//! Tokens when parsing in STRING mode

use location::Span;
use std::fmt;

/// A token of a Nafi string literal
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
        write!(f, "{}({:?})", self.kind, self.source)
    }
}

/// The kind of string token this is
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum Kind {
    /// Literal text
    Text,
    /// An escape sequence
    Escaped(StringEscape),
    /// The start of an interpolated sequence
    InterpolationStart,
    /// The end of a string literal
    StringEnd,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Kind::Text => "Text",
                Kind::Escaped(_) => "Escaped",
                Kind::InterpolationStart => "InterpolationStart",
                Kind::StringEnd => "StringEnd",
            }
        )?;
        if let Kind::Escaped(escape) = self {
            write!(f, "({})", escape)
        } else {
            Ok(())
        }
    }
}

/// The escape sequence kind
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum StringEscape {
    /// U+000D CARRIAGE RETURN ('\r')
    CarriageReturn,
    /// U+000A NEW LINE ('\n')
    NewLine,
    /// U+0009 HORIZONTAL TABULATION (`\t`)
    HorizontalTab,
    /// U+005C REVERSE SOLIDUS (`\\`)
    Backslash,
    /// U+0022 QUOTATION MARK (`\"`)
    Quote,
    /// An escaped Unicode Code Point (`\u{XXXX}`)
    Unicode,
    /// An invalid escape
    Invalid,
}

impl StringEscape {
    /// Get the `StringEscape` for a certain escaped string
    pub fn of(s: &str) -> StringEscape {
        match s {
            "r" => StringEscape::CarriageReturn,
            "n" => StringEscape::NewLine,
            "t" => StringEscape::HorizontalTab,
            "\\" => StringEscape::Backslash,
            "\"" => StringEscape::Quote,
            _ => StringEscape::Invalid,
        }
    }
}

impl fmt::Display for StringEscape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StringEscape::CarriageReturn => "CR",
                StringEscape::NewLine => "NL",
                StringEscape::HorizontalTab => "TAB",
                StringEscape::Backslash => "\\",
                StringEscape::Quote => "\"",
                StringEscape::Unicode => "Unicode",
                StringEscape::Invalid => "INVALID",
            }
        )
    }
}
