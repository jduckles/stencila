// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::delete_inline::DeleteInline;
use super::insert_inline::InsertInline;
use super::modify_inline::ModifyInline;
use super::replace_inline::ReplaceInline;

/// Union type for all types that are descended from `SuggestionInline`
#[derive(Debug, strum::Display, Clone, PartialEq, Serialize, Deserialize, StripNode, WalkNode, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec, WriteNode, ReadNode)]
#[serde(untagged, crate = "common::serde")]
pub enum SuggestionInlineType {
    DeleteInline(DeleteInline),

    InsertInline(InsertInline),

    ModifyInline(ModifyInline),

    ReplaceInline(ReplaceInline),
}