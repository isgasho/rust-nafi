use Span;
use nom::{alpha, digit, InputLength, Slice};
use nom::IResult;
use tokens::BigUint;
use lexer::regex::restore_span;


pub fn white_space(i: Span) -> IResult<Span, Span> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    restore_span(i, re_find_static!(i.fragment, r"^\w+"))
}

pub fn symbol(i: Span) -> IResult<Span, Span> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    restore_span(i, re_find_static!(i.fragment, r"^[\pP\pS]"))
}

pub fn identifier(i: Span) -> IResult<Span, Span> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    restore_span(i, re_find_static!(i.fragment, concat!(
        "^",
        include_str!("xid_start.regex"),
        include_str!("xid_continue.regex"),
        "*"
    )))
}

pub fn decimal_number(i: Span) -> IResult<Span, BigUint> {
    // TODO: Other unicode decimal digits?
    digit(i).map(|(i, o)| (i, o.fragment.parse().unwrap()))
}
