use super::_impl_de::TreeNode;
use super::SyntaxTree;
use serde::de::{Deserialize, Deserializer};

macro_rules! de_kind {
    {
        terminal { $($terminal:ident,)* }
        nonterminal { $($nonterminal:ident,)* }
    } => {
        mod _impl_de {
            use super::{Kind, Node, SyntaxTree};
            use optional::{some, none};

            #[derive(Debug, Deserialize)]
            pub enum TreeNode<'a> {
                $($terminal(&'a str),)*
                $($nonterminal(Vec<TreeNode<'a>>),)*
            }

            impl<'a> From<TreeNode<'a>> for SyntaxTree {
                fn from(tree: TreeNode<'a>) -> Self {
                    match tree {
                        $(TreeNode::$terminal(source) => SyntaxTree {
                            source: source.to_string(),
                            nodes: vec![Node {
                                kind: Kind::$terminal,
                                span: (0, source.len() as u32),
                                parent: none(),
                                child: none(),
                                sibling: none(),
                            }]
                        },)*
                        $(TreeNode::$nonterminal(children) => {
                            let mut source = String::new();
                            let mut nodes = vec![Node {
                                kind: Kind::$nonterminal,
                                span: (0, 0),
                                parent: none(),
                                child: none(),
                                sibling: none(),
                            }];
                            let mut previous_tree = none::<u32>();
                            for subtree in children {
                                let span_offset = source.len() as u32;
                                let node_offset = nodes.len() as u32;
                                if previous_tree.is_some() {
                                    nodes[previous_tree.unpack() as usize].sibling = some(node_offset);
                                } else {
                                    nodes[0].child = some(node_offset);
                                }
                                let mut subtree: SyntaxTree = subtree.into();
                                source.push_str(&subtree.source);
                                for node in &mut subtree.nodes {
                                    node.child = node.child.map_t(|it| it + node_offset);
                                    node.sibling = node.sibling.map_t(|it| it + node_offset);
                                    node.parent = node.parent.map_t(|it| it + node_offset);
                                    node.span = (node.span.0 + span_offset, node.span.1 + span_offset);
                                }
                                subtree.nodes[0].parent = some(0);
                                nodes.extend(subtree.nodes);
                                previous_tree = some(node_offset);
                            }
                            nodes[0].span = (0, source.len() as u32);
                            SyntaxTree { source, nodes }
                        },)*
                    }
                }
            }
        }
    };
}

impl<'de> Deserialize<'de> for SyntaxTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let tree: TreeNode = Deserialize::deserialize(deserializer)?;
        Ok(tree.into())
    }
}
