//! Tokens when parsing in CODE mode

use location::Span;
use std::fmt;

/// A token of Nafi source code
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({:?})@{}", self.kind, self.source, self.span)
    }
}

/// The kind of source token this is
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
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
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Kind::Identifier => "Identifier",
                Kind::Symbol => "Symbol",
                Kind::LiteralInteger => "LiteralInteger",
                Kind::LiteralStringStart => "LiteralStringStart",
                Kind::Whitespace => "Whitespace",
                Kind::Comment(_) => "Comment",
            }
        )?;
        if let Kind::Comment(style) = self {
            write!(f, "({})", style)
        } else {
            Ok(())
        }
    }
}

/// A style of comment
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum CommentStyle {
    /// A comment starting with `//` and ending with a newline (excluded)
    Line,
    /// A comment starting with `///` and ending with a newline (excluded)
    LineDoc,
    /// A comment starting with `/*` and ending with `*/`, nestable
    Block,
    /// A comment starting with `/**` and ending with `*/`, nesting block comments
    BlockDoc,
}

impl fmt::Display for CommentStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CommentStyle::Line => "Line",
                CommentStyle::LineDoc => "LineDoc",
                CommentStyle::Block => "Block",
                CommentStyle::BlockDoc => "BlockDoc",
            }
        )
    }
}
