//! Handling of custom requests for getting the node id for a line
//! and the line for a node id.

use std::sync::Arc;

use async_lsp::{
    lsp_types::{request::Request, Position},
    ResponseError,
};

use common::{
    itertools::Itertools,
    reqwest::Url,
    serde::{Deserialize, Serialize},
    tokio::sync::RwLock,
};

use crate::text_document::TextNode;

pub struct NodeIdsForLines;

impl Request for NodeIdsForLines {
    const METHOD: &'static str = "stencila/nodeIdsForLines";
    type Params = NodeIdsForLinesParams;
    type Result = Vec<Option<String>>;
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "common::serde")]
pub struct NodeIdsForLinesParams {
    /// The URI of the document for which the node ids are desired
    pub uri: Url,

    /// The lines for which corresponding node ids are desired
    pub lines: Vec<u32>,
}

/// Handle a request for getting the node id corresponding to a vector of lines
pub async fn lines_to_node_ids(
    root: Arc<RwLock<TextNode>>,
    lines: Vec<u32>,
) -> Result<Vec<Option<String>>, ResponseError> {
    let root = root.read().await;

    let ids = lines
        .into_iter()
        .map(|line| {
            root.node_id_closest(Position::new(line, 1))
                .map(|node_id| node_id.to_string())
        })
        .collect_vec();

    Ok(ids)
}
