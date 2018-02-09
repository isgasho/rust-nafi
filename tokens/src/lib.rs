//! Tokens of Nafi source

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, missing_docs, unsafe_code, unused)]
#![warn(unreachable_pub)]

#[macro_use]
extern crate derive_deref;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate smart_default;

use std::{fmt, io};

/// Dump tokens to an output stream
pub fn dump<W: io::Write>(tokens: &[Token], mut to: W) -> io::Result<()> {
    writeln!(to, "(")?;
    for token in tokens {
        writeln!(to, "  {}", token)?;
    }
    writeln!(to, ")")?;
    Ok(())
}

// TODO: Move to a more general position?
/// Position in source code
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(From, Into, Constructor)]
#[allow(missing_docs)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A token of source code
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Constructor)]
pub struct Token<'a> {
    position: Position,
    source: &'a str,
    kind: Kind,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}({})@{}", self.kind, self.source.escape_debug(), self.position)
        // FIXME(rust-lang/rust#27791): String::escape_debug is unstable
        write!(f, "{}(", self.kind)?;
        for c in self.source.chars() {
            write!(f, "{}", c.escape_debug())?;
        }
        write!(f, ")@{}", self.position)
    }
}

impl<'a> Token<'a> {
    /// The position in the original source
    pub fn position(&self) -> Position { self.position }
    /// The text from the original source
    pub fn source(&self) -> &'a str { self.source }
    /// The kind of token
    pub fn kind(&self) -> &Kind { &self.kind }
}

/// The kind of token
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(SmartDefault)]
pub enum Kind {
    /// An identifier, matching Unicode UAX31-R1 unmodified (includes keywords)
    Identifier,
    /// A single char symbol, matching Unicode General_Category=Punctuation|Symbol
    Symbol,
    /// A literal integer
    LiteralInteger,
    /// A literal string
    LiteralString,
    /// Whitespace, matching Unicode White_Space
    Whitespace,
    /// Any characters not matched by one of the above cases
    #[doc(hidden)]
    #[default]
    _Unknown,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Kind::Identifier => "Identifier",
                Kind::Symbol => "Symbol",
                Kind::LiteralInteger | Kind::LiteralString => "Literal",
                Kind::Whitespace => "Whitespace",
                Kind::_Unknown => "Unknown",
            }
        )
    }
}

/// A tokenized view of the parts of a string literal
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[derive(Deref, DerefMut)]
pub struct StringFragments<'a>(Vec<StringFragment<'a>>);

impl<'a, S: Into<String>> From<S> for StringFragments<'a> {
    fn from(s: S) -> Self { StringFragments(vec![s.into().into()]) }
}

/// A token in a literal string
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum StringFragment<'a> {
    /// Literal quoted text
    Literal(String),
    /// An escaped character e.g. `\n` or `\u{2744}`
    Escaped(char),
    /// An invalid escape sequence e.g. `\u{bogus}` or `\❄`
    InvalidEscape(Position, &'a str),
    /// An interpolated sequence e.g. `\{name}`
    Interpolated(Vec<Token<'a>>),
    #[doc(hidden)]
    _NonExhaustive,
}

impl<'a, S: Into<String>> From<S> for StringFragment<'a> {
    fn from(s: S) -> Self { StringFragment::Literal(s.into()) }
}
