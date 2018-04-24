use super::{SyntaxTree, Node, Kind};
use serde::de::{Deserialize, Deserializer, Visitor};

struct NodeVisitor;
impl<'de> Visitor<'de> for NodeVisitor {
    type Value = SyntaxTree;

    fn expecting(&self, formatter: &mut _) -> _ {
        "node newtype variant"
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where D: Deserializer<'de> {
        unimplemented!()
    }
}

impl<'de> Deserialize<'de> for SyntaxTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        deserializer.deserialize_enum("Node", Kind.VARIANTS, NodeVisitor)
    }
}
