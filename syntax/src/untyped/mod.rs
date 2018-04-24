use optional::Optioned;

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    source: String,
    nodes: Vec<Node>,
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

macro_rules! kind {
    (pub enum $name:ident { $($variant:ident,)* }) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum $name {
            $($variant,)*
        }

        impl ::std::str::FromStr for $name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, ()> {
                Ok(match s {
                    $(stringify!($variant) => $name::$variant,)*
                    _ => Err(())?,
                })
            }
        }
    };
}

kind! {
pub enum Kind {
    ERROR,

    // Terminals
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

    // Nonterminals
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
    }
}
