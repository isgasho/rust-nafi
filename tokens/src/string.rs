//! Tokens when parsing in STRING mode

use location::Span;

/// A token of a Nafi string literal
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

/// The kind of string token this is
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(SmartDefault)]
pub enum Kind {
    /// Literal text
    Text,
    /// An escape sequence
    Escaped(StringEscape),
    /// The start of an interpolated sequence
    InterpolationStart,
    /// The end of a string literal
    End,
    /// Any characters not matched by one of the above cases
    #[doc(hidden)]
    #[default]
    Unknown,
}

/// The escape sequence kind
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(SmartDefault)]
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
    #[default]
    Invalid,
}
