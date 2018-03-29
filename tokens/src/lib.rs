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
    for token in tokens {
        token.dump(0, &mut to)?;
        writeln!(to)?;
    }
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
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(Constructor)]
pub struct Token<'a> {
    /// The position in the original source
    pub position: Position,
    /// The text from the original source
    pub source: &'a str,
    /// The kind of token
    pub kind: Kind<'a>,
}

impl<'a> Token<'a> {
    fn dump<W: io::Write>(&self, depth: usize, w: &mut W) -> io::Result<()> {
        write!(
            w,
            "{}{}({:?})@{}",
            " ".repeat(depth),
            self.kind,
            self.source,
            self.position
        )?;
        if let Kind::LiteralString(ref pieces) = self.kind {
            for piece in &**pieces {
                writeln!(w)?;
                piece.dump(depth + 1, w)?;
            }
        }
        Ok(())
    }
}

/// The kind of token
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de : 'a"))]
#[derive(SmartDefault)]
pub enum Kind<'a> {
    /// An identifier, matching Unicode UAX31-R1 unmodified (includes keywords)
    Identifier,
    /// A single char symbol, matching Unicode General_Category=Punctuation|Symbol
    Symbol,
    /// A literal integer
    LiteralInteger,
    /// A literal string
    LiteralString(StringFragments<'a>),
    /// Whitespace, matching Unicode White_Space
    Whitespace,
    /// Any characters not matched by one of the above cases
    #[doc(hidden)]
    #[default]
    Unknown,
}

impl<'a> fmt::Display for Kind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Kind::Identifier => "Identifier",
                Kind::Symbol => "Symbol",
                Kind::LiteralInteger | Kind::LiteralString(_) => "Literal",
                Kind::Whitespace => "Whitespace",
                Kind::Unknown => "Unknown",
            }
        )
    }
}

/// A tokenized view of the parts of a string literal
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(bound(deserialize = "'de : 'a"))]
#[derive(Deref, DerefMut)]
pub struct StringFragments<'a>(Vec<StringFragment<'a>>);

impl<'a> From<&'a str> for StringFragments<'a> {
    fn from(s: &'a str) -> Self { StringFragments(vec![s.into()]) }
}

/// A token in a literal string
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum StringFragment<'a> {
    /// Literal quoted text
    Literal(&'a str),
    /// An escaped character e.g. `\n` or `\u{2744}`
    Escaped(char),
    /// An invalid escape sequence e.g. `\u{bogus}` or `\❄`
    InvalidEscape(Position, &'a str),
    /// An interpolated sequence e.g. `\{name}`
    Interpolated(Vec<Token<'a>>),
    #[doc(hidden)]
    _NonExhaustive,
}

impl<'a> From<&'a str> for StringFragment<'a> {
    fn from(s: &'a str) -> Self { StringFragment::Literal(s) }
}

impl<'a> StringFragment<'a> {
    fn dump<W: io::Write>(&self, depth: usize, w: &mut W) -> io::Result<()> {
        let indent = " ".repeat(depth);
        match *self {
            StringFragment::Literal(s) => write!(w, "{}Literal({:?})", indent, s),
            StringFragment::Escaped(c) => write!(w, "{}Escaped({})", indent, c.escape_default()),
            StringFragment::InvalidEscape(pos, s) => {
                write!(w, "{}InvalidEscape({:?})@{}", indent, s, pos)
            },
            StringFragment::Interpolated(ref tokens) => {
                write!(w, "{}Interpolation", indent)?;
                for token in tokens {
                    writeln!(w)?;
                    token.dump(depth + 1, w)?;
                }
                Ok(())
            },
            StringFragment::_NonExhaustive => write!(w, "{}Unknown", indent),
        }
    }
}
