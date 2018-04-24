use super::{SyntaxTree, Node, Kind};
use serde::de::{Deserialize, Deserializer, Visitor};
use std::fmt;

struct NodeVisitor;
impl<'de> Visitor<'de> for NodeVisitor {
    type Value = SyntaxTree;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "node newtype variant")
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where D: Deserializer<'de> {
        unimplemented!()
    }
}

impl<'de> Deserialize<'de> for SyntaxTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        deserializer.deserialize_enum("Node", Kind::VARIANTS, NodeVisitor)
    }
}
