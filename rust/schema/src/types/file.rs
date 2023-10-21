// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::block::Block;
use super::comment::Comment;
use super::creative_work_type::CreativeWorkType;
use super::creative_work_type_or_text::CreativeWorkTypeOrText;
use super::date::Date;
use super::grant_or_monetary_grant::GrantOrMonetaryGrant;
use super::image_object::ImageObject;
use super::inline::Inline;
use super::person::Person;
use super::person_or_organization::PersonOrOrganization;
use super::person_or_organization_or_software_application::PersonOrOrganizationOrSoftwareApplication;
use super::property_value_or_string::PropertyValueOrString;
use super::string::String;
use super::string_or_number::StringOrNumber;
use super::text::Text;
use super::thing_type::ThingType;

/// A file on the file system.
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[derive(derive_more::Display)]
#[display(fmt = "File")]
pub struct File {
    /// The type of this item.
    pub r#type: MustBe!("File"),

    /// The identifier for this item.
    #[strip(metadata)]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// The name of the item.
    #[strip(metadata)]
    pub name: String,

    /// The path (absolute or relative) of the file on the filesystem
    pub path: String,

    /// Non-core optional fields
    #[serde(flatten)]
    #[html(flatten)]
    #[jats(flatten)]
    #[markdown(flatten)]
    pub options: Box<FileOptions>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct FileOptions {
    /// Alternate names (aliases) for the item.
    #[serde(alias = "alternate-names", alias = "alternate_names", alias = "alternateName", alias = "alternate-name", alias = "alternate_name")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub alternate_names: Option<Vec<String>>,

    /// A description of the item.
    #[strip(metadata)]
    pub description: Option<Text>,

    /// Any kind of identifier for any kind of Thing.
    #[serde(alias = "identifier")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub identifiers: Option<Vec<PropertyValueOrString>>,

    /// Images of the item.
    #[serde(alias = "image")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub images: Option<Vec<ImageObject>>,

    /// The URL of the item.
    #[strip(metadata)]
    pub url: Option<String>,

    /// The subject matter of the content.
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub about: Option<Vec<ThingType>>,

    /// A a short description that summarizes a `CreativeWork`.
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub r#abstract: Option<Vec<Block>>,

    /// The authors of the `CreativeWork`.
    #[serde(alias = "author")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub authors: Option<Vec<PersonOrOrganization>>,

    /// A secondary contributor to the `CreativeWork`.
    #[serde(alias = "contributor")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub contributors: Option<Vec<PersonOrOrganizationOrSoftwareApplication>>,

    /// People who edited the `CreativeWork`.
    #[serde(alias = "editor")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub editors: Option<Vec<Person>>,

    /// The maintainers of the `CreativeWork`.
    #[serde(alias = "maintainer")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub maintainers: Option<Vec<PersonOrOrganization>>,

    /// Comments about this creative work.
    #[serde(alias = "comment")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub comments: Option<Vec<Comment>>,

    /// Date/time of creation.
    #[serde(alias = "date-created", alias = "date_created")]
    #[strip(metadata)]
    pub date_created: Option<Date>,

    /// Date/time that work was received.
    #[serde(alias = "date-received", alias = "date_received")]
    #[strip(metadata)]
    pub date_received: Option<Date>,

    /// Date/time of acceptance.
    #[serde(alias = "date-accepted", alias = "date_accepted")]
    #[strip(metadata)]
    pub date_accepted: Option<Date>,

    /// Date/time of most recent modification.
    #[serde(alias = "date-modified", alias = "date_modified")]
    #[strip(metadata)]
    pub date_modified: Option<Date>,

    /// Date of first publication.
    #[serde(alias = "date", alias = "date-published", alias = "date_published")]
    #[strip(metadata)]
    pub date_published: Option<Date>,

    /// People or organizations that funded the `CreativeWork`.
    #[serde(alias = "funder")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub funders: Option<Vec<PersonOrOrganization>>,

    /// Grants that funded the `CreativeWork`; reverse of `fundedItems`.
    #[serde(alias = "funded-by", alias = "funded_by")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub funded_by: Option<Vec<GrantOrMonetaryGrant>>,

    /// Genre of the creative work, broadcast channel or group.
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub genre: Option<Vec<String>>,

    /// Keywords or tags used to describe this content.
    /// Multiple entries in a keywords list are typically delimited by commas.
    #[serde(alias = "keyword")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub keywords: Option<Vec<String>>,

    /// An item or other CreativeWork that this CreativeWork is a part of.
    #[serde(alias = "is-part-of", alias = "is_part_of")]
    #[strip(metadata)]
    pub is_part_of: Option<CreativeWorkType>,

    /// License documents that applies to this content, typically indicated by URL.
    #[serde(alias = "license")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub licenses: Option<Vec<CreativeWorkTypeOrText>>,

    /// Elements of the collection which can be a variety of different elements,
    /// such as Articles, Datatables, Tables and more.
    #[serde(alias = "hasParts", alias = "part")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(content)]
    pub parts: Option<Vec<CreativeWorkType>>,

    /// A publisher of the CreativeWork.
    #[strip(metadata)]
    pub publisher: Option<PersonOrOrganization>,

    /// References to other creative works, such as another publication,
    /// web page, scholarly article, etc.
    #[serde(alias = "citations", alias = "reference")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub references: Option<Vec<CreativeWorkTypeOrText>>,

    /// The textual content of this creative work.
    #[strip(content)]
    pub text: Option<Text>,

    /// The title of the creative work.
    #[serde(alias = "headline")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    #[strip(metadata)]
    pub title: Option<Vec<Inline>>,

    /// The version of the creative work.
    #[strip(metadata)]
    pub version: Option<StringOrNumber>,
}

impl File {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            path,
            ..Default::default()
        }
    }
}
