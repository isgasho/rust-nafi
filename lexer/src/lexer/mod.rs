use tokens::Token;
use nom::{eol, space};

mod literals;
use self::literals::integer_literal;

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

/// Token::_Whitespace
named! {
    _whitespace<&str, Token>,
    do_parse!(
        // TODO: Pattern_White_Space
        many1!(alt_complete!(space | eol)) >>
        (Token::_Whitespace)
    )
}
