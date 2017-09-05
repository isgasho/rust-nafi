/// A symbol in the source code, e.g. `+-={}[]<>` (or others)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Symbol {
    ExclamationMark,
    // QuotationMark, // will never happen -- superseded by string literal
    NumberSign,
    DollarSign,
    PercentSign,
    Ampersand,
    // Apostrophe, // will never happen -- superseded by quote literal
    LeftParenthesis,
    RightParenthesis,
    Asterisk,
    PlusSign,
    Comma,
    HyphenMinus,
    FullStop,
    Solidus,
    Colon,
    Semicolon,
    LessThanSign,
    EqualsSign,
    GreaterThanSign,
    QuestionMark,
    CommercialAt,
    LeftSquareBracket,
    ReverseSolidus,
    RightSquareBracket,
    CircumflexAccent,
    LowLine,
    GraveAccent,
    LeftCurlyBracket,
    VerticalLine,
    RightCurlyBracket,
    Tilde,
    Other(char),
}

impl Symbol {
    /// The character in the source
    pub fn as_char(&self) -> char {
        use Symbol::*;
        match *self {
            ExclamationMark => '!',
            NumberSign => '#',
            DollarSign => '$',
            PercentSign => '%',
            Ampersand => '&',
            LeftParenthesis => '(',
            RightParenthesis => ')',
            Asterisk => '*',
            PlusSign => '+',
            Comma => ',',
            HyphenMinus => '-',
            FullStop => '.',
            Solidus => '/',
            Colon => ':',
            Semicolon => ';',
            LessThanSign => '<',
            EqualsSign => '=',
            GreaterThanSign => '>',
            QuestionMark => '?',
            CommercialAt => '@',
            LeftSquareBracket => '[',
            ReverseSolidus => '\\',
            RightSquareBracket => ']',
            CircumflexAccent => '^',
            LowLine => '_',
            GraveAccent => '`',
            LeftCurlyBracket => '{',
            VerticalLine => '|',
            RightCurlyBracket => '}',
            Tilde => '~',
            Other(char) => char,
        }
    }
}