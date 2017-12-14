use Span;
use nom::{alpha, digit, InputLength, Slice};
use nom::IResult;
use tokens::BigUint;

fn restore_span<'a>(span: Span<'a>, result: IResult<&str, &str>) -> IResult<Span<'a>, Span<'a>> {
    result
        .map(|(_, o)| (span.slice(o.len()..), span.slice(..o.len())))
        .map_err(|e| {
            use nom::{Context, Err};
            match e {
                Err::Error(ctx) => Err::Error(match ctx {
                    Context::Code(i, kind) => Context::Code(span.slice(..i.len()), kind),
                    Context::List(list) => Context::List(
                        list.into_iter()
                            .map(|(o, kind)| (span.slice(..o.len()), kind))
                            .collect(),
                    ),
                }),
                Err::Failure(ctx) => Err::Failure(match ctx {
                    Context::Code(o, kind) => Context::Code(span.slice(..o.len()), kind),
                    Context::List(list) => Context::List(
                        list.into_iter()
                            .map(|(o, kind)| (span.slice(..o.len()), kind))
                            .collect(),
                    ),
                }),
                Err::Incomplete(needed) => Err::Incomplete(needed),
            }
        })
}

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
