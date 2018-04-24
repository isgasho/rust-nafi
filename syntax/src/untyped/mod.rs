use optional::Optioned;

mod ser;
mod de;

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    source: String,
    nodes: Vec<Node>,
}

impl SyntaxTree {
    pub fn root(&self) -> NodeRef {
        self.get(0).expect("Empty SyntaxTree")
    }

    pub fn get(&self, idx: u32) -> Option<NodeRef> {
        self.nodes.get(idx as usize).map(|node| {
            NodeRef { node, syntax: self }
        })
    }
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
    pub fn kind(&self) -> Kind {
        self.node.kind
    }

    pub fn span(&self) -> (u32, u32) {
        (self.node.span.0, self.node.span.1)
    }

    pub fn source(&self) -> &'a str {
        &self.syntax.source[self.span().0 as usize .. self.span().1 as usize]
    }

    pub fn parent(&self) -> Option<NodeRef<'a>> {
        self.syntax.get(self.node.parent.unpack())
    }

    pub fn child(&self) -> Option<NodeRef<'a>> {
        self.syntax.get(self.node.child.unpack())
    }

    pub fn sibling(&self) -> Option<NodeRef<'a>> {
        self.syntax.get(self.node.sibling.unpack())
    }

    pub fn children(&self) -> NodeChildren<'a> {
        NodeChildren(self.child())
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

            pub const VARIANTS: &'static [&'static str] = &[
                $(stringify!($terminal),)*
                $(stringify!($nonterminal),)*
            ];
        }
    };
}

Kind! {
    terminal {
        ERROR,
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
    use ron::ser::to_string_pretty;
    use sexpr::{SExpr, SExprHead, SExprTail, Lit, ParseError};
    use optional::{some, none};
    use std::str::FromStr;

    impl From<SExprHead> for Kind {
        fn from(head: SExprHead) -> Self {
            if let SExprHead::Symbol(ident) = head {
                ident.as_ref().parse().unwrap()
            } else {
                panic!("invalid syntaxtree sexpr")
            }
        }
    }

    impl From<SExpr> for SyntaxTree {
        fn from(expr: SExpr) -> Self {
            match expr {
                SExpr::Tail(_) => panic!("invalid syntaxtree sexpr"),
                SExpr::Pair(_, head, tail) => {
                    let kind: Kind = head.into();
                    let source = if let SExprTail::Literal(Lit::Str(s)) = tail {
                        s.value()
                    } else {
                        panic!("invalid syntaxtree sexpr")
                    };
                    SyntaxTree {
                        nodes: vec![Node {
                            kind,
                            span: (0, source.len() as u32),
                            parent: none(),
                            child: none(),
                            sibling: none(),
                        }], source,
                    }
                }
                SExpr::List(_, head, tail) => {
                    let kind: Kind = head.into();
                    let subtrees: Vec<SyntaxTree> = tail.into_iter().map(|sexpr| sexpr.into()).collect();
                    let mut source = String::new();
                    let mut nodes = vec![Node {
                        kind,
                        span: (0, 0),
                        parent: none(),
                        child: none(),
                        sibling: none(),
                    }];
                    let mut previous_root = None::<u32>;
                    for mut subtree in subtrees {
                        let span_offset = source.len() as u32;
                        let idx_offset = nodes.len() as u32;
                        if let Some(sibling) = previous_root {
                            nodes[sibling as usize].sibling = some(idx_offset);
                        } else {
                            nodes[0].child = some(idx_offset);
                        }
                        source.push_str(&subtree.source);
                        for node in &mut subtree.nodes {
                            node.child = node.child.map_t(|it| it + idx_offset);
                            node.sibling = node.child.map_t(|it| it + idx_offset);
                            node.parent = node.parent.map_t(|it| it + idx_offset);
                            node.span = (node.span.0 + span_offset, node.span.1 + span_offset);
                        }
                        subtree.nodes[0].parent = some(0);
                        nodes.extend(subtree.nodes);
                        previous_root = Some(idx_offset);
                    }
                    nodes[0].span = (0, source.len() as u32);
                    SyntaxTree { source, nodes }
                }
            }
        }
    }

    impl FromStr for SyntaxTree {
        type Err = ParseError;
        fn from_str(s: &str) -> Result<Self, ParseError> {
            Ok(s.parse::<SExpr>()?.into())
        }
    }

    #[test]
    fn if_else_expression() {
        let tree = stringify!(
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
        );
        let tree = stringify!(
            (SideEffect
                (FunctionCall
                    (Identifier "if")
                    (Whitespace " ")
                    (Symbol "(")
                    (Identifier "true")
                    (Symbol ")")
                    (Whitespace " ")
                    (Closure
                        (Symbol "{")
                        (Whitespace "\n    ")
                        (SideEffect
                            (FunctionCall
                                (Identifier "print")
                                (Symbol "(")
                                (FunctionCallArgument
                                    (StringLiteral
                                        (Symbol "\"")
                                        (StringText "true is true")
                                        (Symbol "\"")))
                                (Symbol ")"))
                            (Symbol ";"))
                    (Whitespace "\n")
                    (Symbol "}")))
                (Symbol ";"))
        );
        let tree: SyntaxTree = tree.parse().unwrap();
        println!("{:#?}", tree);
        println!("{}", to_string_pretty(&tree, Default::default()).unwrap());
    }
}
