//! A codec for JSON

use codec_trait::{eyre::Result, stencila_schema::Node, Codec, EncodeOptions};
use node_coerce::coerce;

pub struct JsonCodec {}

impl Codec for JsonCodec {
    fn from_str(str: &str) -> Result<Node> {
        coerce(serde_json::from_str(str)?, None)
    }

    fn to_string(node: &Node, options: Option<EncodeOptions>) -> Result<String> {
        let compact = options.map_or_else(|| false, |options| options.compact);
        let json = match compact {
            true => serde_json::to_string::<Node>(node)?,
            false => serde_json::to_string_pretty::<Node>(node)?,
        };
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codec_trait::stencila_schema::{Paragraph, Primitive};
    use std::collections::BTreeMap;
    use test_utils::assert_debug_eq;

    #[test]
    fn from_str() {
        assert!(matches!(
            JsonCodec::from_str("true").unwrap(),
            Node::Boolean(true)
        ));

        assert!(matches!(
            JsonCodec::from_str("42").unwrap(),
            Node::Integer(42)
        ));

        #[allow(clippy::float_cmp)]
        if let Node::Number(num) = JsonCodec::from_str("1.23").unwrap() {
            assert_eq!(num, 1.23_f64)
        }

        assert!(matches!(
            JsonCodec::from_str("[1, 2, 3]").unwrap(),
            Node::Array(..)
        ));

        assert!(matches!(
            JsonCodec::from_str("{}").unwrap(),
            Node::Object(..)
        ));

        assert!(matches!(
            JsonCodec::from_str("{\"type\": \"Entity\"}").unwrap(),
            Node::Entity(..)
        ));

        assert_debug_eq!(
            JsonCodec::from_str("{\"type\": \"Paragraph\"}").unwrap(),
            Node::Paragraph(Paragraph {
                content: vec![],
                ..Default::default()
            })
        );
    }

    #[test]
    fn to_str() {
        assert_eq!(
            JsonCodec::to_string(&Node::Boolean(true), None).unwrap(),
            "true".to_string()
        );

        assert_eq!(
            JsonCodec::to_string(&Node::Integer(42), None).unwrap(),
            "42".to_string()
        );

        assert_eq!(
            JsonCodec::to_string(&Node::Number(1.23), None).unwrap(),
            "1.23".to_string()
        );

        assert_eq!(
            JsonCodec::to_string(&Node::Array(Vec::new()), None).unwrap(),
            "[]".to_string()
        );

        assert_eq!(
            JsonCodec::to_string(&Node::Object(BTreeMap::new()), None).unwrap(),
            "{}".to_string()
        );

        assert_eq!(
            JsonCodec::to_string(
                &Node::Array(vec![Primitive::Integer(42)]),
                Some(EncodeOptions {
                    compact: false,
                    ..Default::default()
                })
            )
            .unwrap(),
            "[\n  42\n]".to_string()
        );
    }
}
