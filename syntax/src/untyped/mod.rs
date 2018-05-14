use optional::Optioned;

mod ser;

#[derive(Clone, Debug)]
pub struct SyntaxTree {
    pub(crate) source: String,
    pub(crate) nodes: Vec<Node>,
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Span<'a> {
    span: (u32, u32),
    source: &'a str,
}

impl<'a> Span<'a> {
    fn source(&self) -> &'a str {
        &self.source[self.span.0 as usize ..= self.span.1 as usize]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Node {
    pub(crate) kind: Kind,
    pub(crate) span: (u32, u32),
    pub(crate) parent: Optioned<u32>,
    pub(crate) child: Optioned<u32>,
    pub(crate) previous: Optioned<u32>,
    pub(crate) next: Optioned<u32>,
}

#[derive(Copy, Clone, Debug)]
pub struct NodeRef<'a> {
    syntax: &'a SyntaxTree,
    node: &'a Node,
}

impl<'a> NodeRef<'a> {
    pub fn kind(&self) -> Kind { self.node.kind }
    pub fn source(&self) -> &'a str { self.span().source() }
    pub fn span(&self) -> Span<'a> {
        Span {
            span: self.node.span,
            source: &self.syntax.source,
        }
    }

    pub fn parent(&self) -> Option<NodeRef<'a>> { self.syntax.get(self.node.parent.unpack()) }
    pub fn child(&self) -> Option<NodeRef<'a>> { self.syntax.get(self.node.child.unpack()) }
    pub fn previous(&self) -> Option<NodeRef<'a>> { self.syntax.get(self.node.previous.unpack()) }
    pub fn next(&self) -> Option<NodeRef<'a>> { self.syntax.get(self.node.next.unpack()) }
    pub fn children(&self) -> NodeChildren<'a> { NodeChildren(self.child()) }

    /// This is not an implementation of PartialEq because it shouldn't be public
    fn eq_at_and_below(&self, other: &Self) -> bool {
        let simple_eq = self.kind() == other.kind() && self.span() == other.span();
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
        self.0 = next.as_ref().and_then(NodeRef::next);
        next
    }
}

#[allow(bad_style)]
macro_rules! Kind {
    {
        $($kind:ident,)*
    } => {
        #[repr(u32)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum Kind {
            $($kind,)*
        }

        impl ::std::str::FromStr for Kind {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, ()> {
                Ok(match s {
                    $(stringify!($kind) => Kind::$kind,)*
                    _ => Err(())?,
                })
            }
        }

        impl Kind {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Kind::$kind => stringify!($kind),)*
                }
            }
        }
    };
}

Kind! {
    ERROR,
    
    // terminal
    Symbol,
    Identifier,
    DecimalLiteral,
    Whitespace,
    LineDocumentation,
    LineComment,
    BlockDocumentation,
    BlockComment,
    StringText,
    StringEscape,
    
    // nonterminal
    Operator,
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
