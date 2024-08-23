#![recursion_limit = "256"]

use std::{
    fs::{self, read_dir},
    io::Cursor,
    path::{Path, PathBuf},
    time::Duration,
};

use app::{get_app_dir, DirType};
use codec_markdown_trait::to_markdown;
use codecs::{DecodeOptions, EncodeOptions, Format};
use common::{
    chrono::Utc,
    eyre::{eyre, OptionExt},
    futures::future::try_join_all,
    regex::Regex,
    reqwest::Client,
    tar::Archive,
    tokio::{
        fs::{create_dir_all, read_to_string, write},
        time,
    },
};
use flate2::read::GzDecoder;
use images::ensure_http_or_data_uri;
use rust_embed::RustEmbed;

use model::{
    common::{
        derive_more::{Deref, DerefMut},
        eyre::{bail, Result},
        futures::future::join_all,
        itertools::Itertools,
        tokio, tracing,
    },
    schema::{
        authorship, shortcuts::p, Article, AudioObject, Author, AuthorRole, ImageObject, Inline,
        InstructionBlock, InstructionMessage, InstructionType, Link, MessagePart, Node, Prompt,
        SuggestionBlock, SuggestionStatus, Timestamp, VideoObject,
    },
    ModelOutput, ModelOutputKind, ModelTask,
};

pub mod cli;

// Re-export
pub use prompt;

/// An instance of a prompt
///
/// A wrapper around an [`Prompt`] used to cache derived properties
/// such as regexes / embeddings
#[derive(Deref, DerefMut)]
pub struct PromptInstance {
    #[deref]
    #[deref_mut]
    inner: Prompt,

    /// Home directory of the prompt
    ///
    /// Used mainly to resolve relative paths used for the source of
    /// `IncludeBlocks` used within instructions.
    home: PathBuf,

    /// Compiled regexes for the prompt's instruction regexes
    instruction_regexes: Vec<Regex>,
}

impl PromptInstance {
    fn new(inner: Prompt, home: PathBuf) -> Result<Self> {
        let instruction_regexes = inner
            .instruction_patterns
            .iter()
            .flatten()
            .map(|pattern| Regex::new(pattern))
            .try_collect()?;

        let home = home.canonicalize()?;

        Ok(Self {
            inner,
            home,
            instruction_regexes,
        })
    }

    // Get the home of the prompt
    pub fn home(&self) -> PathBuf {
        self.home.clone()
    }
}

/// Get a list of available prompts
///
/// Cached if not in debug mode
#[cfg_attr(not(debug_assertions), cached::proc_macro::cached(time = 3600))]
pub async fn list() -> Vec<PromptInstance> {
    let futures = (0..=3).map(|provider| async move {
        let (provider, result) = match provider {
            0 => ("builtin", list_builtin().await),
            1 => ("local", list_local().await),
            _ => return vec![],
        };

        match result {
            Ok(list) => list,
            Err(error) => {
                tracing::error!("While listing {provider} prompts: {error}");
                vec![]
            }
        }
    });

    join_all(futures).await.into_iter().flatten().collect_vec()
}

/// Builtin prompts
///
/// During development these are loaded directly from the `prompts`
/// directory at the root of the repository but are embedded into the binary on release builds.
#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../../prompts"]
struct Builtin;

/// List the builtin prompts.
pub async fn list_builtin() -> Result<Vec<PromptInstance>> {
    // If in debug mode, just use the prompts dir in the repo
    if cfg!(debug_assertions) {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../prompts");
        return list_dir(&dir).await;
    }

    let dir = get_app_dir(DirType::Prompts, false)?.join("builtin");
    if !dir.exists() {
        create_dir_all(&dir).await?;
    }

    let initialized_at = dir.join("initialized-at.txt");
    let fetched_at = dir.join("fetched-at.txt");

    // If the built-ins have not yet been fetched then write them into
    // the directory. This needs to be done, rather than loading directly from memory
    // (as we used to do) so that inclusions work correctly.
    if !initialized_at.exists() && !fetched_at.exists() {
        let futures = Builtin::iter()
            .filter_map(|name| Builtin::get(&name).map(|file| (name, file.data)))
            .map(|(filename, content)| {
                let dir = dir.clone();
                async move {
                    let path = dir.join(filename.to_string());
                    if let Some(parent) = path.parent() {
                        create_dir_all(parent).await?;
                    }
                    write(path, content).await
                }
            });
        try_join_all(futures).await?;

        // Write timestamp
        write(initialized_at, Utc::now().to_rfc3339()).await?;
    }

    list_dir(&dir).await
}

/// List any local prompts
async fn list_local() -> Result<Vec<PromptInstance>> {
    let dir = get_app_dir(DirType::Prompts, false)?.join("local");

    if dir.exists() {
        list_dir(&dir).await
    } else {
        Ok(Vec::new())
    }
}

/// List prompts in a directory
async fn list_dir(dir: &Path) -> Result<Vec<PromptInstance>> {
    tracing::trace!("Attempting to read prompts from `{}`", dir.display());

    let mut prompts = vec![];
    for entry in read_dir(dir)?.flatten() {
        let path = entry.path();
        let (Some(filename), Some(ext)) = (path.file_name(), path.extension()) else {
            continue;
        };

        let content = read_to_string(&path).await?;

        let node = codecs::from_str(
            &content,
            Some(DecodeOptions {
                format: Some(Format::from_name(&ext.to_string_lossy())),
                ..Default::default()
            }),
        )
        .await?;

        if let Node::Prompt(prompt) = node {
            prompts.push(PromptInstance::new(prompt, dir.to_path_buf())?)
        } else {
            bail!(
                "Expected `{}` to be an `Prompt`, got a `{}`",
                filename.to_string_lossy(),
                node.to_string()
            )
        }
    }

    Ok(prompts)
}

/// Refresh the builtin prompts directory
///
/// So that users can get the latest version of prompts, without installing a new version of
/// the binary. If not updated in last day, creates an `prompts/builtin` app directory
/// (if not yet exists), fetches a tarball of the prompts and extracts it into the directory.
async fn fetch_builtin() -> Result<()> {
    const FETCH_EVERY_SECS: u64 = 12 * 3_600;

    let dir = get_app_dir(DirType::Prompts, true)?.join("builtin");

    // Check for the last time this was done
    let fetched_at = dir.join("fetched-at.txt");
    if let Ok(metadata) = fs::metadata(&fetched_at) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = modified.elapsed() {
                if elapsed < Duration::from_secs(FETCH_EVERY_SECS) {
                    return Ok(());
                }
            }
        }
    }

    tracing::debug!("Updating builtin prompts");

    // Fetch the repo tar ball
    let tar_gz = Client::new()
        .get("https://github.com/stencila/stencila/archive/main.tar.gz")
        .send()
        .await?
        .bytes()
        .await?;
    let tar = GzDecoder::new(Cursor::new(tar_gz));

    // Extract just the builtin prompts
    let mut archive = Archive::new(tar);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if let Ok(relative_path) = path.strip_prefix("stencila-main") {
            if let Ok(name) = relative_path.strip_prefix("prompts/builtin/") {
                entry.unpack(&dir.join(name))?;
            }
        }
    }

    // Write timestamp
    write(fetched_at, Utc::now().to_rfc3339()).await?;

    Ok(())
}

/// Runs `fetch_builtin` in an async background task and log any errors
pub fn update_builtin() {
    tokio::spawn(async {
        loop {
            if let Err(error) = fetch_builtin().await {
                tracing::debug!("While fetching builtin prompts: {error}");
            }
            time::sleep(Duration::from_secs(3_600)).await;
        }
    });
}

/// Select the most appropriate prompt for an instruction
pub async fn select(
    instruction_type: &InstructionType,
    message: &Option<InstructionMessage>,
    assignee: &Option<String>,
    _node_types: &Option<Vec<String>>,
) -> Result<PromptInstance> {
    let prompts = list().await;

    // If there is an assignee then get it
    if let Some(assignee) = assignee {
        let id = if assignee.contains('/') {
            assignee.to_string()
        } else {
            ["stencila/", assignee].concat()
        };

        return prompts
            .into_iter()
            .find(|prompt| prompt.id.as_ref() == Some(&id))
            .ok_or_else(|| eyre!("Unable to find prompt with id `{assignee}`"));
    }

    // Filter the prompts to those that support the instruction type
    let prompts = prompts
        .into_iter()
        .filter(|prompt| prompt.instruction_types.contains(instruction_type));

    // Get the text of the message to match prompts against
    let message_text = match message {
        Some(message) => message
            .parts
            .iter()
            .filter_map(|part| match part {
                MessagePart::Text(text) => Some(text.value.string.clone()),
                _ => None,
            })
            .join(""),
        None => String::new(),
    };

    // Count the number of characters in the instruction message that are matched by
    // each of the patterns in each of the candidates
    let prompt = prompts
        .map(|prompt| {
            let matches = prompt
                .instruction_regexes
                .iter()
                .flat_map(|regex| regex.find_iter(&message_text).map(|found| found.len()))
                .sum::<usize>();
            (prompt, matches)
        })
        .sorted_by(|(.., a), (.., b)| a.cmp(b).reverse())
        .map(|(prompt, ..)| prompt)
        .next()
        .take();

    prompt.ok_or_eyre("No prompts found for instruction")
}

/// Render and prompt's content to Markdown to use as a system prompt
pub async fn render(prompt: PromptInstance) -> Result<String> {
    codecs::to_string(
        &Node::Article(Article {
            content: prompt.content.clone(),
            ..Default::default()
        }),
        Some(EncodeOptions {
            format: Some(Format::Markdown),
            render: Some(true),
            ..Default::default()
        }),
    )
    .await
}

/// Execute an [`InstructionBlock`]
pub async fn execute_instruction_block(
    mut instructors: Vec<AuthorRole>,
    prompter: AuthorRole,
    system_prompt: &str,
    instruction: &InstructionBlock,
    dry_run: bool,
) -> Result<SuggestionBlock> {
    // Create a vector of messages from the system message and instruction messages
    let mut messages = vec![InstructionMessage::system(
        system_prompt,
        Some(vec![Author::AuthorRole(prompter.clone())]),
    )];

    // Ensure that any images in the message are fully resolved
    if let Some(message) = instruction.message.clone() {
        let parts = message
            .parts
            .into_iter()
            .map(|part| {
                Ok(match part {
                    MessagePart::ImageObject(image) => MessagePart::ImageObject(ImageObject {
                        content_url: ensure_http_or_data_uri(&image.content_url)?,
                        ..image
                    }),
                    _ => part,
                })
            })
            .collect::<Result<_>>()?;
        messages.push(InstructionMessage { parts, ..message })
    }

    for suggestion in instruction.suggestions.iter().flatten() {
        // Note: this encodes suggestion content to Markdown. Using the
        // format used by the particular prompt e.g. HTML may be more appropriate
        let md = to_markdown(&suggestion.content);
        messages.push(InstructionMessage::assistant(
            md,
            suggestion.authors.clone(),
        ));

        if let Some(feedback) = &suggestion.feedback {
            messages.push(InstructionMessage::user(feedback, None));
        }
    }

    // Create a model task
    let mut task = ModelTask::new(
        instruction.instruction_type.clone(),
        instruction.model.as_deref().cloned(),
        messages,
    );
    task.dry_run = dry_run;

    // Perform the task
    let started = Timestamp::now();
    let ModelOutput {
        mut authors,
        kind,
        format,
        content,
    } = models::perform_task(task).await?;
    let ended = Timestamp::now();

    let blocks = match kind {
        ModelOutputKind::Text => {
            // Decode the model output into blocks
            let node = codecs::from_str(
                &content,
                Some(DecodeOptions {
                    format: format
                        .is_unknown()
                        .then_some(Format::Markdown)
                        .or(Some(format)),
                    ..Default::default()
                }),
            )
            .await?;

            let Node::Article(Article { content, .. }) = node else {
                bail!("Expected content to be decoded to an article")
            };

            content
        }
        ModelOutputKind::Url => {
            let content_url = content;
            let media_type = Some(format.media_type());

            let node = if format.is_audio() {
                Inline::AudioObject(AudioObject {
                    content_url,
                    media_type,
                    ..Default::default()
                })
            } else if format.is_image() {
                Inline::ImageObject(ImageObject {
                    content_url,
                    media_type,
                    ..Default::default()
                })
            } else if format.is_video() {
                Inline::VideoObject(VideoObject {
                    content_url,
                    media_type,
                    ..Default::default()
                })
            } else {
                Inline::Link(Link {
                    target: content_url,
                    ..Default::default()
                })
            };

            vec![p([node])]
        }
    };

    // TODO: check that blocks are the correct type

    let mut suggestion = SuggestionBlock::new(SuggestionStatus::Proposed, blocks);

    // Record execution time for the suggestion
    let duration = ended
        .duration(&started)
        .expect("should use compatible timestamps");
    suggestion.execution_duration = Some(duration);
    suggestion.execution_ended = Some(ended);

    // Apply authorship to the suggestion.
    authors.append(&mut instructors);
    authors.push(prompter);
    authorship(&mut suggestion, authors)?;

    Ok(suggestion)
}