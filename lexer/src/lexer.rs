use std::str::{self, FromStr};

use tokens::{Token};

use nom::digit;

named!(
    pub token<&str, Token>,
    alt_complete!(
        integer_literal |
        _unknown
    )
);

// == Unknown == //

named!(
    _unknown<&str, Token>, // Token::_Unknown
    do_parse!(
        take!(1) >>
        (Token::_Unknown)
    )
);

// == Literals == //

named!(
    integer_literal<&str, Token>,
    do_parse!(
        i: map_res!(
            digit,
            FromStr::from_str
        ) >>
        (Token::IntegerLiteral(i))
    )
);
