use tokens::string::*;
use {Cursor, Span};

named! { pub text (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        text: spanned_regex!(r#"^[^\\"]+"#) >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            text.fragment.0,
            Kind::Text,
        ))
    )
}

// TODO: unicode escapes `\u{XXXX}`
named! { pub escaped (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        escape: spanned_regex!(r"(?s)^\\.?") >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            escape.fragment.0,
            Kind::Escaped(StringEscape::of(&escape.fragment.0[1..]))
        ))
    )
}

named! { pub interpolation_start (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        open: tag!(r"\{") >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            open.fragment.0,
            Kind::InterpolationStart,
        ))
    )
}

named! { pub end (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        end: tag!("\"") >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            end.fragment.0,
            Kind::StringEnd,
        ))
    )
}
