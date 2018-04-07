use nom::IResult;

use Cursor;

pub(crate) fn white_space(i: Cursor) -> IResult<Cursor, Cursor> {
    spanned_regex!(i, concat!(
        "^[", include_str!("../../resources/white_space.regex"), "]",
    ))
}

pub(crate) fn symbol(i: Cursor) -> IResult<Cursor, Cursor> {
    spanned_regex!(i, r"^[\pP\pS]")
}

pub(crate) fn identifier(i: Cursor) -> IResult<Cursor, Cursor> {
    spanned_regex!(i, concat!(
        "^[", include_str!("../../resources/xid_start.regex"), "]",
        "[", include_str!("../../resources/xid_continue.regex"), "]*",
    ))
}

pub(crate) fn restore_span<'i>(
    span: Cursor<'i>,
    result: IResult<&str, &str>,
) -> IResult<Cursor<'i>, Cursor<'i>> {
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
