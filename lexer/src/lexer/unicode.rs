use Span;
use lexer::regex::restore_span;
use nom::{digit, IResult};
use tokens::BigUint;

#[allow(unused)]
pub fn white_space(i: Span) -> IResult<Span, Span> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    restore_span(i, re_find_static!(i.fragment, r"^\s+"))
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

#[allow(unused)]
pub fn decimal_number(i: Span) -> IResult<Span, BigUint> {
    // TODO: Other unicode decimal digits?
    digit(i).map(|(i, o)| (i, o.fragment.parse().unwrap()))
}
