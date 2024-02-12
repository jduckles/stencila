// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

use super::code_location::CodeLocation;
use super::execution_message_level::ExecutionMessageLevel;
use super::string::String;

/// An error, warning or log message generated executing an executable node.
#[skip_serializing_none]
#[serde_as]
#[derive(Debug, SmartDefault, Clone, PartialEq, Serialize, Deserialize, StripNode, WalkNode, WriteNode, ReadNode, DomCodec, HtmlCodec, JatsCodec, MarkdownCodec, TextCodec)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
#[derive(derive_more::Display)]
#[display(fmt = "ExecutionMessage")]
pub struct ExecutionMessage {
    /// The type of this item.
    pub r#type: MustBe!("ExecutionMessage"),

    /// The identifier for this item.
    #[strip(metadata)]
    #[html(attr = "id")]
    pub id: Option<String>,

    /// The text of the message.
    pub level: ExecutionMessageLevel,

    /// The text of the message.
    pub message: String,

    /// The type of error e.g. "SyntaxError", "ZeroDivisionError".
    #[serde(alias = "error-type", alias = "error_type")]
    pub error_type: Option<String>,

    /// The location that the error occurred or other message emanated from.
    #[serde(alias = "code-location", alias = "code_location")]
    pub code_location: Option<CodeLocation>,

    /// Stack trace leading up to the error.
    #[serde(alias = "trace", alias = "stack-trace", alias = "stack_trace")]
    pub stack_trace: Option<String>,

    /// A unique identifier for a node within a document
    
    #[serde(skip)]
    pub uid: NodeUid
}

impl ExecutionMessage {
    const NICK: &'static str = "exe";
    
    pub fn node_type(&self) -> NodeType {
        NodeType::ExecutionMessage
    }

    pub fn node_id(&self) -> NodeId {
        NodeId::new(Self::NICK, &self.uid)
    }
    
    pub fn new(level: ExecutionMessageLevel, message: String) -> Self {
        Self {
            level,
            message,
            ..Default::default()
        }
    }
}