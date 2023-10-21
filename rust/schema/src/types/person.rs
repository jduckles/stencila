// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::image_object::ImageObject;
use super::organization::Organization;
use super::organization_or_person::OrganizationOrPerson;
use super::postal_address_or_string::PostalAddressOrString;
use super::property_value_or_string::PropertyValueOrString;
use super::string::String;
use super::text::Text;

/// A person (alive, dead, undead, or fictional).
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[derive(derive_more::Display)]
#[display(fmt = "Person")]
pub struct Person {
    /// The type of this item.
    pub r#type: MustBe!("Person"),

    /// The identifier for this item.
    #[strip(metadata)]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// Organizations that the person is affiliated with.
    #[serde(alias = "affiliation")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    pub affiliations: Option<Vec<Organization>>,

    /// Family name. In the U.S., the last name of a person.
    #[serde(alias = "familyName", alias = "surname", alias = "surnames", alias = "lastName", alias = "lastNames", alias = "family-names", alias = "family_names", alias = "family-name", alias = "family_name")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    pub family_names: Option<Vec<String>>,

    /// Given name. In the U.S., the first name of a person.
    #[serde(alias = "firstName", alias = "firstNames", alias = "given-names", alias = "given_names", alias = "givenName", alias = "given-name", alias = "given_name")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    pub given_names: Option<Vec<String>>,

    /// Non-core optional fields
    #[serde(flatten)]
    #[html(flatten)]
    #[jats(flatten)]
    #[markdown(flatten)]
    pub options: Box<PersonOptions>,
}

#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct PersonOptions {
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

    /// The name of the item.
    #[strip(metadata)]
    pub name: Option<String>,

    /// The URL of the item.
    #[strip(metadata)]
    pub url: Option<String>,

    /// Postal address for the person.
    pub address: Option<PostalAddressOrString>,

    /// Email addresses for the person.
    #[serde(alias = "email")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    pub emails: Option<Vec<String>>,

    /// A person or organization that supports (sponsors) something through
    /// some kind of financial contribution.
    #[serde(alias = "funder")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    pub funders: Option<Vec<OrganizationOrPerson>>,

    /// An honorific prefix preceding a person's name such as Dr/Mrs/Mr.
    #[serde(alias = "prefix", alias = "honorific-prefix", alias = "honorific_prefix")]
    pub honorific_prefix: Option<String>,

    /// An honorific suffix after a person's name such as MD/PhD/MSCSW.
    #[serde(alias = "suffix", alias = "honorific-suffix", alias = "honorific_suffix")]
    pub honorific_suffix: Option<String>,

    /// The job title of the person (for example, Financial Manager).
    #[serde(alias = "job-title", alias = "job_title")]
    pub job_title: Option<String>,

    /// An organization (or program membership) to which this person belongs.
    #[serde(alias = "member-of", alias = "member_of")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    pub member_of: Option<Vec<Organization>>,

    /// Telephone numbers for the person.
    #[serde(alias = "telephone", alias = "telephone-numbers", alias = "telephone_numbers", alias = "telephoneNumber", alias = "telephone-number", alias = "telephone_number")]
    #[serde_as(deserialize_as = "Option<OneOrMany<_, PreferMany>>")]
    pub telephone_numbers: Option<Vec<String>>,
}

impl Person {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
