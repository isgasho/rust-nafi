use nom::digit1;
use tokens::code::*;
use {Cursor, Span};

mod comment;

named! { pub identifier (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        ident: spanned_regex!(concat!(
            "^[", include_str!("../../resources/xid_start.regex"), "]",
            "[", include_str!("../../resources/xid_continue.regex"), "]*",
        )) >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            ident.fragment.0,
            Kind::Identifier,
        ))
    )
}

named! { pub symbol (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        symbol: spanned_regex!(r"^[\pP\pS]") >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            symbol.fragment.0,
            Kind::Symbol,
        ))
    )
}

named! { pub literal_integer (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        int: call!(digit1) >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            int.fragment.0,
            Kind::LiteralInteger,
        ))
    )
}

named! { pub literal_string_start (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        quote: tag!("\"") >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            quote.fragment.0,
            Kind::LiteralStringStart,
        ))
    )
}

named! { pub whitespace (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        whitespace: spanned_regex!(concat!(
            "^[", include_str!("../../resources/white_space.regex"), "]+",
        )) >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            whitespace.fragment.0,
            Kind::Whitespace,
        ))
    )
}

named! { pub comment (Cursor) -> Token,
    alt!(
        call!(comment::block_doc) |
        call!(comment::line_doc) |
        call!(comment::block) |
        call!(comment::line)
    )
}
