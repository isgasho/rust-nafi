use nom::IResult;

use Span;

pub fn white_space(i: Span) -> IResult<Span, Span> {
    spanned_regex!(i, concat!(
        "^[", include_str!("../../resources/white_space.regex"), "]",
    ))
}

pub fn symbol(i: Span) -> IResult<Span, Span> {
    spanned_regex!(i, r"^[\pP\pS]")
}

pub fn identifier(i: Span) -> IResult<Span, Span> {
    spanned_regex!(i, concat!(
        "^[", include_str!("../../resources/xid_start.regex"), "]",
        "[", include_str!("../../resources/xid_continue.regex"), "]*",
    ))
}

pub(crate) fn restore_span<'i>(
    span: Span<'i>,
    result: IResult<&str, &str>,
) -> IResult<Span<'i>, Span<'i>> {
    use nom::Slice;
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
