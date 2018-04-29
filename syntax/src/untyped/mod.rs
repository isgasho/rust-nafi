use optional::Optioned;

mod ser;
#[macro_use]
mod de;

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    source: String,
    nodes: Vec<Node>,
}

impl SyntaxTree {
    pub fn root(&self) -> NodeRef { self.get(0).expect("Empty SyntaxTree") }

    pub fn get(&self, idx: u32) -> Option<NodeRef> {
        self.nodes
            .get(idx as usize)
            .map(|node| NodeRef { node, syntax: self })
    }
}

impl Eq for SyntaxTree {}
impl PartialEq for SyntaxTree {
    fn eq(&self, other: &SyntaxTree) -> bool { self.root().eq_at_and_below(&other.root()) }
}

#[derive(Copy, Clone, Debug)]
pub struct Node {
    kind: Kind,
    span: (u32, u32),
    parent: Optioned<u32>,
    child: Optioned<u32>,
    sibling: Optioned<u32>,
}

#[derive(Copy, Clone, Debug)]
pub struct NodeRef<'a> {
    syntax: &'a SyntaxTree,
    node: &'a Node,
}

impl<'a> NodeRef<'a> {
    pub fn kind(&self) -> Kind { self.node.kind }

    pub fn span(&self) -> (u32, u32) { (self.node.span.0, self.node.span.1) }

    pub fn source(&self) -> &'a str {
        &self.syntax.source[self.span().0 as usize..self.span().1 as usize]
    }

    pub fn parent(&self) -> Option<NodeRef<'a>> { self.syntax.get(self.node.parent.unpack()) }

    pub fn child(&self) -> Option<NodeRef<'a>> { self.syntax.get(self.node.child.unpack()) }

    pub fn sibling(&self) -> Option<NodeRef<'a>> { self.syntax.get(self.node.sibling.unpack()) }

    pub fn children(&self) -> NodeChildren<'a> { NodeChildren(self.child()) }

    /// This is not an implementation of PartialEq because it shouldn't be public
    fn eq_at_and_below(&self, other: &Self) -> bool {
        let simple_eq = self.kind() == other.kind() && self.span() == other.span()
            && self.source() == other.source();
        if !simple_eq {
            return false;
        }

        self.children().count() == other.children().count()
            && self.children()
                .zip(other.children())
                .all(|(lhs, rhs)| lhs.eq_at_and_below(&rhs))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct NodeChildren<'a>(Option<NodeRef<'a>>);

impl<'a> Iterator for NodeChildren<'a> {
    type Item = NodeRef<'a>;
    fn next(&mut self) -> Option<NodeRef<'a>> {
        let next = self.0.take();
        self.0 = next.as_ref().and_then(NodeRef::sibling);
        next
    }
}

#[allow(bad_style)]
macro_rules! Kind {
    {
        terminal { $($terminal:ident,)* }
        nonterminal { $($nonterminal:ident,)* }
    } => {
        #[repr(u32)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum Kind {
            $($terminal,)*
            $($nonterminal,)*
        }

        impl ::std::str::FromStr for Kind {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, ()> {
                Ok(match s {
                    $(stringify!($terminal) => Kind::$terminal,)*
                    $(stringify!($nonterminal) => Kind::$nonterminal,)*
                    _ => Err(())?,
                })
            }
        }

        impl Kind {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Kind::$terminal => stringify!($terminal),)*
                    $(Kind::$nonterminal => stringify!($nonterminal),)*
                }
            }

            pub fn is_terminal(&self) -> bool {
                match self {
                    $(Kind::$terminal)|* => true,
                    $(Kind::$nonterminal)|* => false,
                }
            }

            pub fn is_nonterminal(&self) -> bool {
                match self {
                    $(Kind::$terminal)|* => false,
                    $(Kind::$nonterminal)|* => true,
                }
            }
        }

        de_kind! {
            terminal { $($terminal,)* }
            nonterminal { $($nonterminal,)* }
        }
    };
}

Kind! {
    terminal {
        Symbol,
        Identifier,
        IntegerLiteral,
        Whitespace,
        LineDocumentation,
        LineComment,
        BlockDocumentation,
        BlockComment,
        StringText,
        StringEscape,
    }
    nonterminal {
        ERROR,
        BinaryOperation,
        PrefixOperation,
        SuffixOperation,
        Parenthesized,
        StringLiteral,
        FunctionCall,
        FunctionCallArgument,
        Closure,
        ClosureArgument,
        Declaration,
        Assignment,
        SideEffect,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ron::de::from_str;
    use ron::ser::to_string;
    use std::str::FromStr;

    #[test]
    fn if_else_expression() {
        let tree = r#"
            SideEffect([
                FunctionCall([
                    Identifier("if"),
                    Whitespace(" "),
                    Symbol("("),
                    Identifier("true"),
                    Symbol(")"),
                    Whitespace(" "),
                    Closure([
                        Symbol("{"),
                        Whitespace("\n    "),
                        SideEffect([
                            FunctionCall([
                                Identifier("print"),
                                Symbol("("),
                                FunctionCallArgument([
                                    StringLiteral([
                                        Symbol("\""),
                                        StringText("true is true"),
                                        Symbol("\""),
                                    ]),
                                ]),
                                Symbol(")"),
                            ]),
                            Symbol(";"),
                        ]),
                        Whitespace("\n"),
                        Symbol("}")
                    ]),
                ]),
                Symbol(";"),
            ])
        "#;
        let tree1: SyntaxTree = from_str(tree).unwrap();
        let serialized = to_string(&tree1).unwrap();
        let tree2: SyntaxTree = from_str(&serialized).unwrap();
        assert_eq!(tree1, tree2);
    }
}
