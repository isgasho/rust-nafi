use tokens::Token;

mod literals;
mod whitespace;

use self::literals::integer_literal;
use self::whitespace::_whitespace;

/// Vec<Token>
named! {
    pub tokens<&str, Vec<Token>>,
    many0!(token)
}

/// Token
named! {
    token<&str, Token>,
    alt_complete!(
        integer_literal |
        _whitespace |
        _unknown
    )
}

/// Token::_Unknown
named! {
    _unknown<&str, Token>,
    do_parse!(
        take!(1) >>
        (Token::_Unknown)
    )
}
