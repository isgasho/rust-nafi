//! Tokens when parsing in CODE mode

use location::Span;

/// A token of Nafi source code
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Constructor)]
pub struct Token<'a> {
    /// The position in the original source
    pub span: Span,
    /// The text from the original source
    pub source: &'a str,
    /// The kind of token
    pub kind: Kind,
}

/// The kind of source token this is
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(SmartDefault)]
pub enum Kind {
    /// An identifier, matching Unicode UAX31-R1 unmodified (includes keywords)
    Identifier,
    /// A single char symbol, matching Unicode `General_Category=Punctuation|Symbol`
    Symbol,
    /// A literal integer
    LiteralInteger,
    /// A literal string
    LiteralStringStart,
    /// Whitespace; characters with Unicode property `White_Space=yes`
    Whitespace,
    /// A comment in the source.
    Comment(CommentStyle),
    /// Any characters not matched by one of the above cases
    #[doc(hidden)]
    #[default]
    Unknown,
}

/// A style of comment
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum CommentStyle {
    /// A comment starting with `//` and ending with a newline
    Line,
    /// A comment starting with `///` and ending with a newline
    LineDoc,
    /// A comment starting with `/*` and ending with `*/`, nestable
    Block,
    /// A comment starting with `/**` and ending with `*/`, nesting block comments
    BlockDoc,
}
