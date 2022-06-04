use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    hash::Hasher,
    io,
    os::unix::prelude::MetadataExt,
    path::{Path, PathBuf},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::Utc;
use eyre::{bail, eyre, Result};
use jwalk::WalkDirGeneric;
use oci_spec::image::{
    Descriptor, DescriptorBuilder, History, HistoryBuilder, ImageConfiguration,
    ImageConfigurationBuilder, ImageIndexBuilder, ImageManifestBuilder, MediaType, RootFsBuilder,
    SCHEMA_VERSION,
};
use seahash::SeaHasher;

// Serialization framework defaults to `rkyv` with fallback to `serde` JSON

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize, Serialize};

#[cfg(feature = "rkyv-safe")]
use bytecheck::CheckBytes;

#[cfg(not(feature = "rkyv"))]
use serde::{Deserialize, Serialize};

use archive_utils::{flate2, tar, zstd};
use hash_utils::{sha2::Digest, sha2::Sha256, str_sha256_hex};
use http_utils::tempfile::{tempdir, TempDir};

use crate::{
    distribution::{Client, DOCKER_REGISTRY},
    media_types::ToDockerV2S2,
    utils::unique_string,
};

#[derive(Debug, Default, PartialEq, serde::Serialize)]
pub struct ImageReference {
    /// The registry the image is on. Defaults to `registry.hub.docker.com`
    pub registry: String,

    /// The repository the image is in e.g. `ubuntu`, `library/hello-world`
    pub repository: String,

    /// An image tag e.g. `sha256:...`. Conflicts with `digest`.
    pub tag: Option<String>,

    /// An image digest e.g. `sha256:e07ee1baac5fae6a26f3...`. Conflicts with `tag`.
    pub digest: Option<String>,
}

impl ImageReference {
    /// Get the digest or tag for the reference, falling back to `latest`
    ///
    /// Use this when pulling a manifest to get the version that most closely
    /// matches that specified in the reference.
    pub fn digest_or_tag_or_latest(&self) -> String {
        match self.digest.as_ref().or_else(|| self.tag.as_ref()) {
            Some(reference) => reference.clone(),
            None => "latest".to_string(),
        }
    }

    /// Get the tag for the reference falling back to `latest`
    ///
    /// Use this when pushing a manifest for the image.
    pub fn tag_or_latest(&self) -> String {
        self.tag.clone().unwrap_or_else(|| "latest".to_string())
    }

    /// Convert reference to a string with `tag` or "latest" (i.e. not using any `digest`).
    pub fn to_string_tag_or_latest(&self) -> String {
        [
            &self.registry,
            "/",
            &self.repository,
            ":",
            self.tag.as_deref().unwrap_or("latest"),
        ]
        .concat()
    }
}

impl FromStr for ImageReference {
    type Err = eyre::Report;

    /// Parse a string into an [`ImageSpec`]
    ///
    /// Based on the implementation in https://github.com/HewlettPackard/dockerfile-parser-rs/
    fn from_str(str: &str) -> Result<ImageReference> {
        let parts: Vec<&str> = str.splitn(2, '/').collect();

        let first = parts[0];
        let (registry, rest) = if parts.len() == 2
            && (first == "localhost" || first.contains('.') || first.contains(':'))
        {
            (Some(parts[0]), parts[1])
        } else {
            (None, str)
        };

        let registry = if matches!(registry, None) || matches!(registry, Some("docker.io")) {
            DOCKER_REGISTRY.to_string()
        } else {
            registry
                .expect("Should be Some because of the match above")
                .to_string()
        };

        let (name, tag, hash) = if let Some(at_pos) = rest.find('@') {
            let (name, hash) = rest.split_at(at_pos);
            (name.to_string(), None, Some(hash[1..].to_string()))
        } else {
            let parts: Vec<&str> = rest.splitn(2, ':').collect();
            let name = parts[0].to_string();
            let tag = parts.get(1).map(|str| str.to_string());
            (name, tag, None)
        };

        let name = if registry == DOCKER_REGISTRY && !name.contains('/') {
            ["library/", &name].concat()
        } else {
            name
        };

        Ok(ImageReference {
            registry,
            repository: name,
            tag,
            digest: hash,
        })
    }
}

impl ToString for ImageReference {
    fn to_string(&self) -> String {
        if let Some(digest) = &self.digest {
            [&self.registry, "/", &self.repository, "@", digest].concat()
        } else {
            self.to_string_tag_or_latest()
        }
    }
}

/// A container image
///
/// This is serializable mainly so that it can be inspected as JSON or YAML output from a CLI command.
#[derive(Debug, serde::Serialize)]
pub struct Image {
    /// The working directory to build an image for
    ///
    /// Buildpacks will build layers based on the source code within this directory. Usually
    /// the home directroy of a project. Defaults to the current directory.
    working_dir: Option<PathBuf>,

    /// The image reference for this image
    #[serde(rename = "ref")]
    ref_: ImageReference,

    /// The image reference for the base image from which this image is derived
    ///
    /// Equivalent to the `FROM` directive of a Dockerfile.
    base: ImageReference,

    /// The directory that contains the buildpack layers
    ///
    /// Defaults to `/layers` or `<working_dir>/.stencila/layers` (in order of priority).
    layers_dir: PathBuf,

    /// Whether snapshots should be diffed or replicated
    layer_diffs: bool,

    /// The format used when writing layers
    layer_format: MediaType,

    /// The snapshots for each layer directory, used to generated [`ChangeSet`]s and image layers
    #[serde(skip)]
    layer_snapshots: Vec<Snapshot>,

    /// The directory where this image will be written to
    ///
    /// The image will be written to this directory following the [OCI Image Layout Specification]
    /// (https://github.com/opencontainers/image-spec/blob/main/image-layout.md)
    layout_dir: PathBuf,

    /// Whether the layout directory should be written will all layers, including those of the base image
    ///
    /// When pushing an image to a registry, if the registry already has a base layer, it is not
    /// necessary to pull it first. However, in some cases it may be desirable to have all layers included.
    layout_complete: bool,

    /// The temporary directory created for the duration of the image's life to write layout to
    #[serde(skip)]
    #[allow(dead_code)]
    layout_tempdir: Option<TempDir>,

    /// The format for the image manifest
    ///
    /// Defaults to `application/vnd.oci.image.manifest.v1+json`. However, for some registries it
    /// may be necessary to use `application/vnd.docker.distribution.manifest.v2+json` (which has
    /// the same underlying schema).
    manifest_format: String,
}

impl Image {
    /// Create a new image
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        working_dir: Option<&Path>,
        ref_: Option<&str>,
        base: Option<&str>,
        layers_dir: Option<&Path>,
        layer_diffs: Option<bool>,
        layer_format: Option<&str>,
        layout_dir: Option<&Path>,
        layout_complete: bool,
        manifest_format: Option<&str>,
    ) -> Result<Self> {
        let working_dir = working_dir.map(PathBuf::from);

        let ref_ = match ref_ {
            Some(reference) => reference.parse::<ImageReference>()?,
            None => {
                let registry = DOCKER_REGISTRY.to_string();

                let name = working_dir
                    .as_ref()
                    .and_then(|dir| dir.file_name())
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unnamed".to_string());
                let hash = working_dir
                    .as_ref()
                    .map(|dir| str_sha256_hex(&dir.to_string_lossy().to_string()))
                    .unwrap_or_else(unique_string);
                let repository = [&name, "-", &hash[..12]].concat();

                ImageReference {
                    registry,
                    repository,
                    ..Default::default()
                }
            }
        };

        let base = base
            .map(String::from)
            .or_else(|| std::env::var("STENCILA_IMAGE_REF").ok())
            .unwrap_or_else(|| "stencila/stencila:nano".to_string())
            .parse()?;

        let layers_dir = layers_dir
            .map(|path| path.to_path_buf())
            .unwrap_or_else(|| {
                let layers_top = PathBuf::from("/layers");
                if layers_top.exists() {
                    layers_top
                } else if let Some(working_dir) = working_dir.as_ref() {
                    let dir = working_dir.join(".stencila").join("layers");
                    std::fs::create_dir_all(&dir).expect("Unable to create layers dir");
                    dir
                } else {
                    std::env::temp_dir().join(["stencila-", &unique_string()].concat())
                }
            });

        // Before creating snapshots do a "prebuild" so that the directories
        // that may need to be snapshotted are present and picked up in `layers_dir.read_dir()` call below.
        buildpacks::PACKS.prebuild_all(&layers_dir)?;

        let mut layer_snapshots = Vec::new();
        if let Some(working_dir) = working_dir.as_ref() {
            layer_snapshots.push(Snapshot::new(working_dir.clone(), "/workspace"));
        }
        for subdir in layers_dir.read_dir()?.flatten().filter_map(|entry| {
            if entry.path().is_dir() {
                Some((entry.path(), entry.file_name()))
            } else {
                None
            }
        }) {
            layer_snapshots.push(Snapshot::new(
                &subdir.0,
                PathBuf::from("/layers").join(subdir.1),
            ));
        }

        let (layout_dir, layout_tempdir) = match layout_dir {
            Some(path) => (PathBuf::from(path), None),
            None => {
                let tempdir = tempdir()?;
                (tempdir.path().to_path_buf(), Some(tempdir))
            }
        };

        let layer_diffs = layer_diffs.unwrap_or(true);

        let layer_format = match layer_format {
            None | Some("tar+gzip") | Some("tgz") => MediaType::ImageLayerGzip,
            Some("tar+zstd") | Some("tzs") => MediaType::ImageLayerZstd,
            Some("tar") => MediaType::ImageLayer,
            _ => bail!("Unknown layer format"),
        };

        let manifest_format = match manifest_format {
            None | Some("oci") => MediaType::ImageManifest.to_string(),
            Some("v2s2") => MediaType::ImageManifest.to_docker_v2s2()?.to_string(),
            _ => bail!("Unknown manifest format"),
        };

        Ok(Self {
            working_dir,
            ref_,
            base,
            layers_dir,
            layer_snapshots,
            layer_diffs,
            layer_format,
            layout_dir,
            layout_complete,
            layout_tempdir,
            manifest_format,
        })
    }

    /// Get the [`ImageReference`] of the image
    pub fn reference(&self) -> &ImageReference {
        &self.ref_
    }

    /// Get the [`ImageReference`] of the image's base
    pub fn base(&self) -> &ImageReference {
        &self.base
    }

    /// Get the the image's OCI layout directory
    pub fn layout_dir(&self) -> &Path {
        &self.layout_dir
    }

    /// Fetches the manifest and configuration of the base image
    ///
    /// Used when writing the image because the DiffIDs (from the config) and the layers (from the
    /// manifest) and required for the config and manifest of this image.
    async fn get_base(&self) -> Result<(String, ImageConfiguration, Vec<Descriptor>)> {
        let client = Client::new(&self.base.registry, &self.base.repository, None).await?;
        let (manifest, digest) = client
            .get_manifest(self.base.digest_or_tag_or_latest())
            .await?;

        let config = client.get_config(&manifest).await?;
        let layers = manifest.layers().clone();

        Ok((digest, config, layers))
    }

    /// Write the image layer blobs and returns vectors of DiffIDs and layer descriptors
    async fn write_layers(
        &self,
        base_config: &ImageConfiguration,
        base_layers: Vec<Descriptor>,
    ) -> Result<(Vec<Descriptor>, Vec<String>, Vec<History>)> {
        let mut layers = base_layers;
        let mut diff_ids = base_config.rootfs().diff_ids().clone();
        let mut histories = base_config.history().clone();

        if self.layout_complete {
            let client = Client::new(&self.base.registry, &self.base.repository, None).await?;
            for layer in &layers {
                client.pull_blob_via(&self.layout_dir, layer).await?
            }
        }

        for snapshot in &self.layer_snapshots {
            let (diff_id, layer) =
                snapshot.write_layer(&self.layout_dir, self.layer_diffs, &self.layer_format)?;

            if diff_id == "<empty>" {
                continue;
            }

            diff_ids.push(diff_id);
            layers.push(layer);

            let history = HistoryBuilder::default()
                .created(Utc::now().to_rfc3339())
                .created_by(format!(
                    "stencila {}",
                    env::args().skip(1).collect::<Vec<String>>().join(" ")
                ))
                .comment(format!("Layer for directory {}", snapshot.source_dir))
                .build()?;
            histories.push(history)
        }

        Ok((layers, diff_ids, histories))
    }

    /// Write the image config blob
    ///
    /// Implements the [OCI Image Configuration Specification](https://github.com/opencontainers/image-spec/blob/main/config.md).
    fn write_config(
        &self,
        base_config: &ImageConfiguration,
        diff_ids: Vec<String>,
        history: Vec<History>,
    ) -> Result<Descriptor> {
        // Start with the config of the base image and override as necessary
        let mut config = base_config.config().clone().unwrap_or_default();

        // Working directory is represented in image as /workspace. Override it
        config.set_working_dir(Some("/workspace".to_string()));

        let layers_dir = self
            .layers_dir
            .to_str()
            .ok_or_else(|| eyre!("Layers dir is none"))?;

        // Get the environment variables in the base images
        let mut env: HashMap<String, String> = config
            .env()
            .clone()
            .unwrap_or_default()
            .iter()
            .map(|name_value| {
                let mut parts = name_value.splitn(2, '=');
                (
                    parts.next().unwrap_or_default().to_owned(),
                    parts.next().unwrap_or_default().to_owned(),
                )
            })
            .collect();

        // Update the env vars with those that are expected to be provided by the `launcher` lifecycle
        // See https://github.com/buildpacks/spec/blob/main/buildpack.md#provided-by-the-lifecycle
        let layer_dirs = glob::glob(&[layers_dir, "/*/*/"].concat())?.flatten();
        for layer_dir in layer_dirs {
            let path = [
                layer_dir.join("bin").to_string_lossy().to_string(),
                ":".to_string(),
                env.get("PATH").cloned().unwrap_or_default(),
            ]
            .concat();
            env.insert("PATH".to_string(), path);

            let lid_library_path = [
                layer_dir.join("lib").to_string_lossy().to_string(),
                ":".to_string(),
                env.get("LD_LIBRARY_PATH").cloned().unwrap_or_default(),
            ]
            .concat();
            env.insert("LD_LIBRARY_PATH".to_string(), lid_library_path);
        }

        // Update the env vars with those provided by buildpacks
        // See https://github.com/buildpacks/spec/blob/main/buildpack.md#provided-by-the-buildpacks
        let var_files = glob::glob(&[layers_dir, "/*/*/env/*"].concat())?.flatten();
        for var_file in var_files {
            let action = match var_file.extension() {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => continue,
            };
            let name = match var_file.file_stem() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            };
            let mut value = match env.get(&name) {
                Some(value) => value.clone(),
                None => String::new(),
            };
            let new_value = std::fs::read_to_string(&var_file)?;

            // Apply modification action
            // Because the base image may have been built with Stencila buildpacks, for
            // prepend and append the value is only added if it is not present (this avoid
            // having env vars such as PATH that grow very long).
            match action.as_str() {
                "default" => {
                    if value.is_empty() {
                        value = new_value;
                    }
                }
                "prepend" => {
                    if !value.contains(&new_value) {
                        value = [new_value, value].concat()
                    }
                }
                "append" => {
                    if !value.contains(&new_value) {
                        value = [value, new_value].concat()
                    }
                }
                "override" => {
                    value = new_value;
                }
                _ => tracing::warn!("ignoring env var file {}", var_file.display()),
            }

            env.insert(name, value);
        }

        // Add an env var for the ref of the image (used as the default `--from` image when building another image from this)
        env.insert("STENCILA_IMAGE_REF".to_string(), self.ref_.to_string());

        let env: Vec<String> = env
            .iter()
            .map(|(name, value)| [name, "=", value].concat())
            .collect();
        config.set_env(Some(env));

        // Extend labels, including with the contents of an y `.image-labels` file in working dir
        let mut labels = config.labels().clone().unwrap_or_default();
        labels.insert(
            "io.stencila.version".to_string(),
            env!("CARGO_PKG_VERSION").to_string(),
        );
        if let Some(content) = self
            .working_dir
            .as_ref()
            .and_then(|dir| std::fs::read_to_string(dir.join(".image-labels")).ok())
        {
            for line in content.lines() {
                if let Some((name, value)) = line.split_once(' ') {
                    labels.insert(name.into(), value.into());
                }
            }
        }
        config.set_labels(Some(labels));

        // Rootfs DiffIDs calculated above
        let rootfs = RootFsBuilder::default().diff_ids(diff_ids).build()?;

        let configuration = ImageConfigurationBuilder::default()
            .created(Utc::now().to_rfc3339())
            .os(env::consts::OS)
            // Not that arch should be one of the values listed at https://go.dev/doc/install/source#environment
            // and that `env::consts::ARCH` does not necessarily return that
            .architecture("amd64")
            .config(config)
            .rootfs(rootfs)
            .history(history)
            .build()?;

        BlobWriter::write_json(
            &self.layout_dir,
            MediaType::ImageConfig,
            &configuration,
            None,
        )
    }

    /// Write the image manifest blob
    ///
    /// Implements the [OCI Image Manifest Specification](https://github.com/opencontainers/image-spec/blob/main/manifest.md).
    /// Given that the manifest requires the descriptors for config and layers also calls `write_config` and `write_layers`.
    async fn write_manifest(&self) -> Result<(String, Descriptor)> {
        let (base_digest, base_config, base_layers) = self.get_base().await?;

        let (layers, diff_ids, history) = self.write_layers(&base_config, base_layers).await?;

        let config = self.write_config(&base_config, diff_ids, history)?;

        let manifest = ImageManifestBuilder::default()
            .schema_version(SCHEMA_VERSION)
            .media_type(self.manifest_format.as_str())
            .config(config)
            .layers(layers)
            .build()?;

        Ok((
            base_digest,
            BlobWriter::write_json(&self.layout_dir, MediaType::ImageManifest, &manifest, None)?,
        ))
    }

    /// Write the image `index.json`
    ///
    /// Implements the [OCI Image Index Specification](https://github.com/opencontainers/image-spec/blob/main/image-index.md).
    /// Updates both `self.ref_.digest` and `self.base.digest`.
    async fn write_index(&mut self) -> Result<()> {
        use tokio::fs;

        let (base_digest, manifest) = self.write_manifest().await?;

        self.base.digest = Some(base_digest.clone());
        self.ref_.digest = Some(manifest.digest().to_string());

        let annotations: HashMap<String, String> = [
            // Where appropriate use pre defined annotations
            // https://github.com/opencontainers/image-spec/blob/main/annotations.md#pre-defined-annotation-keys
            (
                "org.opencontainers.image.ref.name".to_string(),
                self.ref_.to_string_tag_or_latest(),
            ),
            (
                "org.opencontainers.image.created".to_string(),
                Utc::now().to_rfc3339(),
            ),
            (
                "org.opencontainers.image.base.name".to_string(),
                self.base.to_string_tag_or_latest(),
            ),
            (
                "org.opencontainers.image.base.digest".to_string(),
                base_digest,
            ),
        ]
        .into();

        let index = ImageIndexBuilder::default()
            .schema_version(SCHEMA_VERSION)
            .media_type(MediaType::ImageIndex)
            .manifests([manifest])
            .annotations(annotations)
            .build()?;
        fs::write(
            self.layout_dir.join("index.json"),
            serde_json::to_string_pretty(&index)?,
        )
        .await?;

        Ok(())
    }

    pub async fn build(&self) -> Result<()> {
        if let Some(working_dir) = &self.working_dir {
            // Because buildpacks will change directories into the working dir. It is safest to use absolute paths here.
            let working_dir = working_dir.canonicalize()?;
            let layers_dir = self.layers_dir.canonicalize()?;

            buildpacks::PACKS.build_all(Some(&working_dir), Some(&layers_dir), None)?;
        }

        Ok(())
    }

    /// Write the image to `layout_dir`
    ///
    /// Implements the [OCI Image Layout Specification](https://github.com/opencontainers/image-spec/blob/main/image-layout.md).
    ///
    /// Note that the `blobs/sha256` subdirectory may not have blobs for the base image (these
    /// are only pulled into that directory if necessary i.e. if the registry does not yet have them).
    pub async fn write(&mut self) -> Result<()> {
        use tokio::fs;

        if self.layout_dir.exists() {
            fs::remove_dir_all(&self.layout_dir).await?;
        }
        fs::create_dir_all(&self.layout_dir).await?;

        self.write_index().await?;

        fs::write(
            self.layout_dir.join("oci-layout"),
            r#"{"imageLayoutVersion": "1.0.0"}"#,
        )
        .await?;

        Ok(())
    }

    /// Push the image to its registry
    ///
    /// The image must be written first (by a call to `self.write()`).
    pub async fn push(&self) -> Result<()> {
        let client = Client::new(&self.ref_.registry, &self.ref_.repository, None).await?;
        client
            .push_image(&self.ref_.tag_or_latest(), &self.layout_dir)
            .await?;

        Ok(())
    }
}

/// The set of changes between two snapshots
///
/// Represents the set of changes between two filesystem snapshots as described in
/// [OCI Image Layer Filesystem Changeset](https://github.com/opencontainers/image-spec/blob/main/layer.md)
struct ChangeSet {
    /// The source directory, on the local filesystem, for the changes
    source_dir: PathBuf,

    /// The destination directory, within the image's root filesystem, for the changes
    dest_dir: PathBuf,

    /// The change items
    items: Vec<Change>,
}

impl ChangeSet {
    /// Create a new set of snapshot changes
    fn new<P: AsRef<Path>>(source_dir: P, dest_dir: P, items: Vec<Change>) -> Self {
        let source_dir = source_dir.as_ref().to_path_buf();

        // Parths in tar archive must be relative so stri any leading slash
        let dest_dir = dest_dir.as_ref().to_path_buf();
        let dest_dir = match dest_dir.strip_prefix("/") {
            Ok(dir) => dir.to_owned(),
            Err(_) => dest_dir,
        };

        Self {
            source_dir,
            dest_dir,
            items,
        }
    }

    /// Get the number of changes in this set
    fn len(&self) -> usize {
        self.items.len()
    }

    /// Creates an OCI layer for the set of changes
    ///
    /// This implements the [Representing Changes](https://github.com/opencontainers/image-spec/blob/main/layer.md#representing-changes)
    /// section of the OCI image spec:
    ///
    /// - `Added` and `Modified` paths are added to the archive.
    /// - `Removed` paths are represented as "whiteout" files.
    ///
    /// Note that two SHA256 hashes are calculated, one for the `DiffID` of a changeset (calculated in this function
    /// and used in the image config file) and one for the digest which (calculated by the [`BlobWriter`] and used in the image manifest).
    /// A useful diagram showing how these are calculated and used is available
    /// [here](https://github.com/google/go-containerregistry/blob/main/pkg/v1/remote/README.md#anatomy-of-an-image-upload).
    ///
    /// # Arguments
    ///
    /// - `layout_dir`: the image directory to write the layer to (to the `blob/sha256` subdirectory)
    fn write_layer<P: AsRef<Path>>(
        self,
        layout_dir: P,
        media_type: &MediaType,
    ) -> Result<(String, Descriptor)> {
        if self.len() == 0 {
            return Ok((
                "<empty>".to_string(),
                DescriptorBuilder::default()
                    .media_type(media_type.clone())
                    .digest("<none>")
                    .size(0)
                    .build()?,
            ));
        }

        tracing::info!(
            "Writing image layer from changeset for `{}`",
            self.source_dir.display()
        );

        let mut diffid_hash = Sha256::new();
        let mut blob_writer = BlobWriter::new(&layout_dir, media_type.to_owned())?;

        let changes = self.len();
        let mut additions: Vec<String> = Vec::new();
        let mut modifications: Vec<String> = Vec::new();
        let mut deletions: Vec<String> = Vec::new();

        {
            enum LayerEncoder<'lt> {
                Plain(&'lt mut BlobWriter),
                Gzip(flate2::write::GzEncoder<&'lt mut BlobWriter>),
                Zstd(zstd::stream::AutoFinishEncoder<'lt, &'lt mut BlobWriter>),
            }

            struct LayerWriter<'lt> {
                diffid_hash: &'lt mut Sha256,
                layer_encoder: LayerEncoder<'lt>,
            }

            impl<'lt> io::Write for LayerWriter<'lt> {
                fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                    self.diffid_hash.update(buf);
                    match &mut self.layer_encoder {
                        LayerEncoder::Plain(encoder) => encoder.write_all(buf)?,
                        LayerEncoder::Gzip(encoder) => encoder.write_all(buf)?,
                        LayerEncoder::Zstd(encoder) => encoder.write_all(buf)?,
                    }
                    Ok(buf.len())
                }

                fn flush(&mut self) -> io::Result<()> {
                    Ok(())
                }
            }

            let layer_encoder = match media_type {
                MediaType::ImageLayer => LayerEncoder::Plain(&mut blob_writer),
                MediaType::ImageLayerGzip => LayerEncoder::Gzip(flate2::write::GzEncoder::new(
                    &mut blob_writer,
                    flate2::Compression::new(4),
                )),
                MediaType::ImageLayerZstd => LayerEncoder::Zstd(
                    zstd::stream::Encoder::new(&mut blob_writer, 4)?.auto_finish(),
                ),
                _ => bail!("Unhandled media type for layer: {}", media_type.to_string()),
            };

            let mut layer_writer = LayerWriter {
                diffid_hash: &mut diffid_hash,
                layer_encoder,
            };

            let mut archive = tar::Builder::new(&mut layer_writer);

            // Add an entry for the `dest_dir` so that ownership and other metadata is maintained
            archive.append_path_with_name(&self.source_dir, &self.dest_dir)?;

            // Add each change
            for change in self.items {
                match change {
                    Change::Added(ref path) | Change::Modified(ref path) => {
                        if let Err(error) = archive.append_path_with_name(
                            self.source_dir.join(path),
                            self.dest_dir.join(path),
                        ) {
                            tracing::debug!(
                                "While adding added or modified entry to layer: {}",
                                error
                            )
                        } else {
                            match change {
                                Change::Added(..) => additions.push(path.to_string()),
                                Change::Modified(..) => modifications.push(path.to_string()),
                                _ => unreachable!(),
                            }
                        }
                    }
                    Change::Removed(path) => {
                        let path_buf = PathBuf::from(&path);
                        let basename = path_buf
                            .file_name()
                            .ok_or_else(|| eyre!("Path has no file name"))?;
                        let mut whiteout = OsString::from(".wh.".to_string());
                        whiteout.push(basename);
                        let path_buf = match path_buf.parent() {
                            Some(parent) => parent.join(whiteout),
                            None => PathBuf::from(whiteout),
                        };
                        let path_buf = self.dest_dir.join(path_buf);

                        let mut header = tar::Header::new_gnu();
                        header.set_path(path_buf)?;
                        header.set_size(0);
                        header.set_cksum();
                        let data: &[u8] = &[];

                        if let Err(error) = archive.append(&header, data) {
                            tracing::debug!("While adding whiteout entry for file: {}", error)
                        } else {
                            deletions.push(path)
                        }
                    }
                };
            }
        }

        let diff_id = format!("sha256:{:x}", diffid_hash.finalize());

        let mut annotations: HashMap<String, String> = [
            ("io.stencila.version", env!("CARGO_PKG_VERSION").to_string()),
            ("io.stencila.layer.created", Utc::now().to_rfc3339()),
            (
                "io.stencila.layer.directory",
                self.dest_dir.to_string_lossy().to_string(),
            ),
            ("io.stencila.layer.changes", changes.to_string()),
        ]
        .map(|(name, value)| (name.to_string(), value))
        .into();

        fn first_100(vec: Vec<String>) -> String {
            vec[..(std::cmp::min(vec.len(), 100))].join(":")
        }
        if !additions.is_empty() {
            annotations.insert(
                "io.stencila.layer.additions".to_string(),
                first_100(additions),
            );
        }
        if !modifications.is_empty() {
            annotations.insert(
                "io.stencila.layer.modifications".to_string(),
                first_100(modifications),
            );
        }
        if !deletions.is_empty() {
            annotations.insert(
                "io.stencila.layer.deletions".to_string(),
                first_100(deletions),
            );
        }

        let descriptor = blob_writer.finish(Some(annotations))?;

        Ok((diff_id, descriptor))
    }

    /// Get the path of a layer blob within an image directory
    ///
    /// # Arguments
    ///
    /// - `image_dir`: the image directory
    /// - `digest`: the digest of the layer (with or without the "sha256:" prefix)
    fn layer_path<P: AsRef<Path>>(image_dir: P, digest: &str) -> PathBuf {
        image_dir
            .as_ref()
            .join("blobs")
            .join("sha256")
            .join(digest.strip_prefix("sha256:").unwrap_or(digest))
    }

    /// Read a layer blob (a compressed tar archive) from an image directory
    ///
    /// At this stage, mainly just used for testing.
    ///
    /// # Arguments
    ///
    /// - `image_dir`: the image directory
    /// - `digest`: the digest of the layer (with or without the "sha256:" prefix)
    fn read_layer<P: AsRef<Path>>(
        image_dir: P,
        digest: &str,
    ) -> Result<tar::Archive<flate2::read::GzDecoder<std::fs::File>>> {
        use std::fs;

        let path = Self::layer_path(image_dir, digest);
        let file = fs::File::open(&path)?;
        let decoder = flate2::read::GzDecoder::new(file);
        let archive = tar::Archive::new(decoder);
        Ok(archive)
    }
}

/// A change in a path between two snapshots
///
/// This enum represents the [Change Types](https://github.com/opencontainers/image-spec/blob/main/layer.md#change-types)
/// described in the OCI spec.
#[derive(Debug, PartialEq)]
enum Change {
    Added(String),
    Modified(String),
    Removed(String),
}

/// A snapshot of the files and directories in a directory
///
/// A snapshot is created at the start of a session and stored to disk. Another snapshot
/// is taken at the end of session. The changes between the snapshots are used to create
/// an image layer.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "rkyv", derive(Archive))]
#[cfg_attr(feature = "rkyv-safe", archive_attr(derive(CheckBytes)))]
struct Snapshot {
    /// The source directory, on the local filesystem, for the snapshot
    source_dir: String,

    /// The destination directory, on the image's root filesystem, for the snapshot
    dest_dir: String,

    /// Entries in the snapshot
    entries: HashMap<String, SnapshotEntry>,
}

impl Snapshot {
    /// Create a new snapshot of a directory
    ///
    /// If there is a `.dockerignore` or `.containerignore` file in source directory then it will be
    /// used to exclude paths, including those in child sub-directories.
    fn new<S: AsRef<Path>, D: AsRef<Path>>(source_dir: S, dest_dir: D) -> Self {
        let source_dir = source_dir.as_ref().to_path_buf();
        let dest_dir = dest_dir.as_ref().to_path_buf();

        let docker_ignore = source_dir.join(".dockerignore");
        let container_ignore = source_dir.join(".containerignore");
        fn parse_ignore_file(path: &Path) -> Option<gitignore::File> {
            match gitignore::File::new(&path) {
                Ok(file) => Some(file),
                Err(error) => {
                    tracing::warn!(
                        "Error while parsing `{}`; will not be used: {}",
                        path.display(),
                        error
                    );
                    None
                }
            }
        }
        let ignore_file = if docker_ignore.exists() {
            parse_ignore_file(&docker_ignore)
        } else if container_ignore.exists() {
            parse_ignore_file(&container_ignore)
        } else {
            None
        };

        let entries = WalkDirGeneric::<((), SnapshotEntry)>::new(&source_dir)
            .skip_hidden(false)
            .process_read_dir(|_depth, _path, _read_dir_state, children| {
                children.iter_mut().flatten().for_each(|dir_entry| {
                    if !dir_entry.file_type.is_dir() {
                        dir_entry.client_state = SnapshotEntry::new(
                            &dir_entry.path(),
                            &dir_entry.file_type(),
                            dir_entry.metadata().ok(),
                        );
                    }
                })
            })
            .into_iter()
            .filter_map(|entry_result| match entry_result {
                Ok(entry) => {
                    let path = entry.path();

                    // Check that the file should not be ignored
                    if let Some(true) = ignore_file
                        .as_ref()
                        .map(|ignore_file| ignore_file.is_excluded(&path).ok())
                        .flatten()
                    {
                        return None;
                    };

                    let relative_path = path
                        .strip_prefix(&source_dir)
                        .expect("Should always be able to strip the root dir");
                    match relative_path == PathBuf::from("") {
                        true => None, // This is the entry for the dir itself so ignore it
                        false => Some((
                            relative_path.to_string_lossy().to_string(), // Should be lossless on Linux (and MacOS)
                            entry.client_state,
                        )),
                    }
                }
                Err(error) => {
                    tracing::error!("While snapshotting `{}`: {}", source_dir.display(), error);
                    None
                }
            })
            .collect();

        let source_dir = source_dir.to_string_lossy().to_string();
        let dest_dir = dest_dir.to_string_lossy().to_string();
        Self {
            source_dir,
            dest_dir,
            entries,
        }
    }

    /// Create a new snapshot by repeating the current one
    fn repeat(&self) -> Self {
        Self::new(&self.source_dir, &self.dest_dir)
    }

    /// Write a snapshot to a file
    fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        use std::fs;

        #[cfg(feature = "rkyv")]
        {
            let bytes = rkyv::to_bytes::<Self, 256>(self)?;
            fs::write(path, bytes)?;
        }

        #[cfg(not(feature = "rkyv"))]
        {
            let json = serde_json::to_string_pretty(self)?;
            fs::write(path, json)?;
        }

        Ok(())
    }

    /// Read a snapshot from a file
    fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        use std::fs;

        #[cfg(feature = "rkyv")]
        {
            let bytes = fs::read(path)?;

            #[cfg(feature = "rkyv-safe")]
            let archived = match rkyv::check_archived_root::<Self>(&bytes[..]) {
                Ok(archived) => archived,
                Err(error) => {
                    bail!("While checking archive: {}", error)
                }
            };

            #[cfg(not(feature = "rkyv-safe"))]
            let archived = unsafe { rkyv::archived_root::<Self>(&bytes[..]) };

            let snapshot = archived.deserialize(&mut rkyv::Infallible)?;
            Ok(snapshot)
        }

        #[cfg(not(feature = "rkyv"))]
        {
            let json = fs::read_to_string(&path)?;
            let snapshot = serde_json::from_str(&json)?;
            Ok(snapshot)
        }
    }

    /// Create a set of changes that replicate the current snapshot using only additions
    fn replicate(&self) -> ChangeSet {
        let changes = self
            .entries
            .keys()
            .map(|path| Change::Added(path.into()))
            .collect();
        ChangeSet::new(&self.source_dir, &self.dest_dir, changes)
    }

    /// Create a set of changes by determining the difference between two snapshots
    fn diff(&self, other: &Snapshot) -> ChangeSet {
        let mut changes = Vec::new();
        for (path, entry) in self.entries.iter() {
            match other.entries.get(path) {
                Some(other_entry) => {
                    if entry != other_entry {
                        changes.push(Change::Modified(path.into()))
                    }
                }
                None => changes.push(Change::Removed(path.into())),
            }
        }
        for path in other.entries.keys() {
            if !self.entries.contains_key(path) {
                changes.push(Change::Added(path.into()))
            }
        }
        ChangeSet::new(&self.source_dir, &self.dest_dir, changes)
    }

    /// Create a set of changes by repeating the current snapshot
    ///
    /// Convenience function for combining calls to `repeat` and `diff.
    fn changes(&self) -> ChangeSet {
        self.diff(&self.repeat())
    }

    /// Create a layer by repeating the current snapshot
    ///
    /// # Arguments
    ///
    /// - `diff`: Whether to create the layer as the difference to the original snapshot
    ///           (the usual) or as a replicate.
    fn write_layer<P: AsRef<Path>>(
        &self,
        layout_dir: P,
        diff: bool,
        media_type: &MediaType,
    ) -> Result<(String, Descriptor)> {
        let new = self.repeat();
        let changeset = if diff {
            self.diff(&new)
        } else {
            new.replicate()
        };
        changeset.write_layer(layout_dir, media_type)
    }
}

/// An entry for a file or directory in a snapshot
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "rkyv", derive(Archive))]
#[cfg_attr(feature = "rkyv-safe", archive_attr(derive(CheckBytes)))]
struct SnapshotEntry {
    /// Metadata on the file or directory
    ///
    /// Should only be `None` if there was an error getting the metadata
    /// while creating the snapshot.
    metadata: Option<SnapshotEntryMetadata>,

    /// Hash of the content of the file
    ///
    /// Will be `None` if the entry is a directory
    fingerprint: Option<u64>,
}

/// Filesystem metadata for a snapshot entry
///
/// Only includes the metadata that needs to be differences. For that reason,
/// does not record `modified` time since that would create a false positive
/// difference (if all other attributes were the same).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "rkyv", derive(Archive))]
#[cfg_attr(feature = "rkyv-safe", archive_attr(derive(CheckBytes)))]
struct SnapshotEntryMetadata {
    uid: u32,
    gid: u32,
    readonly: bool,
}

impl SnapshotEntry {
    /// Create a new snapshot entry
    fn new(
        path: &Path,
        file_type: &std::fs::FileType,
        metadata: Option<std::fs::Metadata>,
    ) -> Self {
        let metadata = metadata.map(|metadata| SnapshotEntryMetadata {
            uid: metadata.uid(),
            gid: metadata.gid(),
            readonly: metadata.permissions().readonly(),
        });

        let fingerprint = if file_type.is_file() {
            match Self::file_fingerprint::<SeaHasher>(path) {
                Ok(fingerprint) => Some(fingerprint),
                Err(error) => {
                    tracing::error!("While fingerprinting file `{}`: {}", path.display(), error);
                    None
                }
            }
        } else {
            None
        };

        Self {
            metadata,
            fingerprint,
        }
    }

    /// Generate a hash of a file's content
    ///
    /// Used to generate a fingerprint
    ///
    /// Based on https://github.com/jRimbault/yadf/blob/04205a57882ffa7d6a9ca05016e18214a38079b6/src/fs/hash.rs#L29
    fn file_fingerprint<H>(path: &Path) -> io::Result<u64>
    where
        H: Hasher + Default,
    {
        struct HashWriter<H>(H);
        impl<H: Hasher> io::Write for HashWriter<H> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0.write(buf);
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let mut hasher = HashWriter(H::default());
        io::copy(&mut std::fs::File::open(path)?, &mut hasher)?;
        Ok(hasher.0.finish())
    }

    /// Get a timestamp from a file's created or modified system time
    fn file_timestamp(time: Result<SystemTime, io::Error>) -> Option<u64> {
        time.map(|system_time| {
            system_time
                .duration_since(UNIX_EPOCH)
                .expect("Time should not go backwards")
                .as_secs()
        })
        .ok()
    }
}

/// A writer that calculates the size and SHA256 hash of files as they are written
///
/// Writes blobs into the `blobs/sha256` subdirectory of an image directory and returns
/// an [OCI Content Descriptor](https://github.com/opencontainers/image-spec/blob/main/descriptor.md).
///
/// Allows use to do a single pass when writing files instead of reading them after writing in order
/// to generate the SHA256 signature.
struct BlobWriter {
    /// The path to the `blobs/sha256` subdirectory where the blob is written to
    blobs_dir: PathBuf,

    /// The media type of the blob
    media_type: MediaType,

    /// The temporary file name of the blob (used before we know its final name, which is its SHA256 checksum)
    file_name: PathBuf,

    /// The file the blob is written to
    file: std::fs::File,

    /// The number of bytes in the blob content
    bytes: usize,

    /// The SHA256 hash of the blob content
    hash: Sha256,
}

impl BlobWriter {
    /// Create a new blob writer
    ///
    /// # Arguments
    ///
    /// - `image_dir`: the image directory (blobs are written to the `blobs/sha256` subdirectory of this)
    /// - `media_type`: the media type of the blob
    fn new<P: AsRef<Path>>(image_dir: P, media_type: MediaType) -> Result<Self> {
        use std::fs::{self, File};

        let blobs_dir = image_dir.as_ref().join("blobs").join("sha256");
        fs::create_dir_all(&blobs_dir)?;

        let filename = PathBuf::from(format!("temporary-{}", unique_string()));
        let file = File::create(blobs_dir.join(&filename))?;

        Ok(Self {
            blobs_dir,
            media_type,
            file_name: filename,
            file,
            bytes: 0,
            hash: Sha256::new(),
        })
    }

    /// Finish writing the blob
    ///
    /// Finalizes the SHA256 hash, renames the file to the hex digest of that hash,
    /// and returns a descriptor of the blob.
    fn finish(self, annotations: Option<HashMap<String, String>>) -> Result<Descriptor> {
        use std::fs;

        let sha256 = format!("{:x}", self.hash.finalize());

        fs::rename(
            self.blobs_dir.join(self.file_name),
            self.blobs_dir.join(&sha256),
        )?;

        let mut descriptor = DescriptorBuilder::default()
            .media_type(self.media_type)
            .size(self.bytes as i64)
            .digest(format!("sha256:{}", sha256));
        if let Some(annotations) = annotations {
            descriptor = descriptor.annotations(annotations)
        }
        let descriptor = descriptor.build()?;

        Ok(descriptor)
    }

    /// Write an object as a JSON based media type
    fn write_json<P: AsRef<Path>, S: serde::Serialize>(
        path: P,
        media_type: MediaType,
        object: &S,
        annotations: Option<HashMap<String, String>>,
    ) -> Result<Descriptor> {
        let mut writer = Self::new(path, media_type)?;
        serde_json::to_writer_pretty(&mut writer, object)?;
        writer.finish(annotations)
    }
}

impl io::Write for BlobWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write_all(buf)?;
        self.bytes += buf.len();
        self.hash.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use hash_utils::file_sha256_hex;
    use test_snaps::fixtures;
    use test_utils::{print_logs, tempfile::tempdir};

    use super::*;

    /// Test parsing image spec
    #[test]
    fn parse_image_ref() -> Result<()> {
        let ubuntu = ImageReference {
            registry: "registry.hub.docker.com".to_string(),
            repository: "library/ubuntu".to_string(),
            ..Default::default()
        };

        assert_eq!("ubuntu".parse::<ImageReference>()?, ubuntu);
        assert_eq!("docker.io/ubuntu".parse::<ImageReference>()?, ubuntu);
        assert_eq!(
            "registry.hub.docker.com/ubuntu".parse::<ImageReference>()?,
            ubuntu
        );

        let ubuntu_2204 = ImageReference {
            registry: "registry.hub.docker.com".to_string(),
            repository: "library/ubuntu".to_string(),
            tag: Some("22.04".to_string()),
            ..Default::default()
        };

        assert_eq!("ubuntu:22.04".parse::<ImageReference>()?, ubuntu_2204);
        assert_eq!(
            "docker.io/ubuntu:22.04".parse::<ImageReference>()?,
            ubuntu_2204
        );
        assert_eq!(
            "registry.hub.docker.com/ubuntu:22.04".parse::<ImageReference>()?,
            ubuntu_2204
        );

        let ubuntu_digest = ImageReference {
            registry: "registry.hub.docker.com".to_string(),
            repository: "library/ubuntu".to_string(),
            digest: Some("sha256:abcdef".to_string()),
            ..Default::default()
        };

        assert_eq!(
            "ubuntu@sha256:abcdef".parse::<ImageReference>()?,
            ubuntu_digest
        );
        assert_eq!(
            "docker.io/ubuntu@sha256:abcdef".parse::<ImageReference>()?,
            ubuntu_digest
        );
        assert_eq!(
            "registry.hub.docker.com/ubuntu@sha256:abcdef".parse::<ImageReference>()?,
            ubuntu_digest
        );

        Ok(())
    }

    /// Test that snapshots are correctly written to and read back from disk
    #[test]
    fn snapshot_serialization() -> Result<()> {
        let working_dir = fixtures().join("projects").join("apt");

        let temp = tempdir()?;
        let snapshot_path = temp.path().join("test.snap");
        let snapshot1 = Snapshot::new(working_dir, "/workspace");

        snapshot1.write(&snapshot_path)?;

        let snapshot2 = Snapshot::read(&snapshot_path)?;
        assert_eq!(snapshot1, snapshot2);

        Ok(())
    }

    /// Test that snap-shotting takes into consideration .dockerignore and .containerignore files
    #[test]
    fn snapshot_ignores() -> Result<()> {
        use std::fs;

        let temp = tempdir()?;
        let source_dir = temp.path();

        fs::write(source_dir.join("a.txt"), "A")?;
        assert!(Snapshot::new(source_dir, "").entries.contains_key("a.txt"));

        fs::write(source_dir.join(".dockerignore"), "*\n")?;
        assert!(!Snapshot::new(source_dir, "").entries.contains_key("a.txt"));

        fs::remove_file(source_dir.join(".dockerignore"))?;
        fs::write(source_dir.join(".containerignore"), "*.txt\n")?;
        assert!(!Snapshot::new(source_dir, "").entries.contains_key("a.txt"));

        fs::remove_file(source_dir.join(".containerignore"))?;
        fs::write(source_dir.join("b.txt"), "B")?;
        fs::write(source_dir.join(".dockerignore"), "!a.txt\n")?;
        let snapshot = Snapshot::new(source_dir, "");
        assert!(snapshot.entries.contains_key("a.txt"));
        assert!(snapshot.entries.contains_key("b.txt"));

        Ok(())
    }

    /// Test snap-shotting, calculation of changesets, and the generation of layers from them.
    #[test]
    fn snapshot_changes() -> Result<()> {
        use std::fs;

        print_logs();

        // Create a temporary directory as a text fixture and a tar file for writing / reading layers

        let source_dir = tempdir()?;
        let dest_dir = PathBuf::from("workspace");
        let image_dir = tempdir()?;

        // Create an initial snapshot which should be empty and has no changes with self

        let snap1 = Snapshot::new(source_dir.path(), &dest_dir);
        assert_eq!(snap1.entries.len(), 0);

        let changes = snap1.diff(&snap1);
        assert_eq!(changes.len(), 0);

        // Add a file, create a new snapshot and check it has one entry and produces a change set
        // with `Added` and tar has entry for it

        let a_txt = "a.txt".to_string();
        fs::write(source_dir.path().join(&a_txt), "Hello from a.txt")?;

        let snap2 = snap1.repeat();
        assert_eq!(snap2.entries.len(), 1);
        assert_eq!(snap2.entries[&a_txt].fingerprint, Some(3958791156379554752));

        let changes = snap1.diff(&snap2);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes.items[0], Change::Added(a_txt.clone()));

        let (.., descriptor) = changes.write_layer(&image_dir, &MediaType::ImageLayerGzip)?;

        let mut layer = ChangeSet::read_layer(&image_dir, descriptor.digest())?;
        let mut entries = layer.entries()?;
        let entry = entries
            .next()
            .ok_or_else(|| eyre!("No entries in tar archive"))??;
        assert_eq!(entry.path()?, dest_dir.join(&a_txt));
        assert_eq!(entry.size(), 16);

        // Repeat

        let b_txt = "b.txt".to_string();
        fs::write(source_dir.path().join(&b_txt), "Hello from b.txt")?;

        let snap3 = snap1.repeat();
        assert_eq!(snap3.entries.len(), 2);
        assert_eq!(snap2.entries[&a_txt].fingerprint, Some(3958791156379554752));
        assert_eq!(
            snap3.entries[&b_txt].fingerprint,
            Some(15548480638800185371)
        );

        let changes = snap2.diff(&snap3);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes.items[0], Change::Added(b_txt.clone()));

        // Remove a.txt and check that the change set has a `Removed` and tar has
        // a whiteout entry of size 0

        fs::remove_file(source_dir.path().join(&a_txt))?;

        let snap4 = snap1.repeat();
        assert_eq!(snap4.entries.len(), 1);
        assert_eq!(
            snap4.entries[&b_txt].fingerprint,
            Some(15548480638800185371)
        );

        let changes = snap3.diff(&snap4);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes.items[0], Change::Removed(a_txt));

        let (.., descriptor) = changes.write_layer(&image_dir, &MediaType::ImageLayerGzip)?;
        let mut layer = ChangeSet::read_layer(&image_dir, descriptor.digest())?;
        let mut entries = layer.entries()?;
        let entry = entries.next().unwrap()?;
        assert_eq!(entry.path()?, dest_dir.join(".wh.a.txt"));
        assert_eq!(entry.size(), 0);

        // Modify b.txt and check that the change set has a `Modified` and tar has
        // entry with new content

        fs::write(source_dir.path().join(&b_txt), "Hello")?;

        let snap5 = snap1.repeat();
        assert_eq!(snap5.entries.len(), 1);
        assert_eq!(snap5.entries[&b_txt].fingerprint, Some(3297469917561599766));

        let changes = snap4.diff(&snap5);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes.items[0], Change::Modified(b_txt.clone()));

        let (.., descriptor) = changes.write_layer(&image_dir, &MediaType::ImageLayerGzip)?;
        let mut archive = ChangeSet::read_layer(&image_dir, descriptor.digest())?;
        let mut entries = archive.entries()?;
        let entry = entries.next().unwrap()?;
        assert_eq!(entry.path()?, dest_dir.join(b_txt));
        assert_eq!(entry.size(), 5);

        Ok(())
    }

    /// Test that the descriptor for a layer is accurate (SHA256 and size are same as
    /// when independently calculated)
    #[test]
    fn changes_layer() -> Result<()> {
        use std::fs;

        let source_dir = tempdir()?;
        let image_dir = tempdir()?;

        let snap = Snapshot::new(&source_dir, "workspace");

        fs::write(&source_dir.path().join("some-file.txt"), "Hello")?;

        // Create a layer archive, diffid and descriptor

        let changes = snap.changes();
        let (diff_id, descriptor) = changes.write_layer(&image_dir, &MediaType::ImageLayerGzip)?;

        assert_eq!(diff_id.len(), 7 + 64);
        assert!(diff_id.starts_with("sha256:"));

        // Test that size and digest in the descriptor is as for the file
        let archive = image_dir
            .path()
            .join("blobs")
            .join("sha256")
            .join(descriptor.digest().strip_prefix("sha256:").unwrap());

        let size = fs::metadata(&archive)?.len() as i64;
        assert_eq!(descriptor.size(), size);

        let digest = format!("sha256:{}", file_sha256_hex(&archive)?);
        assert_eq!(descriptor.digest(), &digest);

        Ok(())
    }

    /// Test that when an image is written to a directory that they directory conforms to
    /// the OCI Image Layout spec
    #[tokio::test]
    async fn image_write() -> Result<()> {
        let working_dir = tempdir()?;
        let mut image = Image::new(
            Some(working_dir.path()),
            None,
            None,
            None,
            None,
            None,
            None,
            false,
            None,
        )?;

        image.write().await?;

        assert!(image.layout_dir.join("oci-layout").is_file());
        assert!(image.layout_dir.join("index.json").is_file());
        assert!(image.layout_dir.join("blobs").join("sha256").is_dir());

        Ok(())
    }
}
