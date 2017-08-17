use std::str::FromStr;
use tokens::Token;
use nom::digit;

/// Token::IntegerLiteral
named! {
    pub integer_literal<&str, Token>,
    do_parse!(
        i: map_res!(digit, FromStr::from_str) >>
        (Token::IntegerLiteral(i))
    )
}
