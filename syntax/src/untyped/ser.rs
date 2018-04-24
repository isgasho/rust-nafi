use super::{SyntaxTree, NodeRef, NodeChildren};
use serde::ser::{Serialize, Serializer};

impl Serialize for SyntaxTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        self.root().serialize(serializer)
    }
}

impl<'a> Serialize for NodeRef<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let kind = self.kind();
        if self.child().is_some() {
            serializer.serialize_newtype_variant(
                "Node",
                kind as u32,
                kind.as_str(),
                &self.children(),
            )
        } else {
            let source = self.source();
            serializer.serialize_newtype_variant(
                "Node",
                kind as u32,
                kind.as_str(),
                source,
            )
        }
    }
}

impl<'a> Serialize for NodeChildren<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(None)?;
        for node in *self {
            seq.serialize_element(&node)?;
        }
        seq.end()
    }
}
