use nom::IResult;
use Cursor;

// Without this I get an extra } at the end of the code line !?
#[cfg_attr(rustfmt, rustfmt_skip)]
macro_rules! spanned_regex {
    ($i:ident, $re:expr) => {
        ::utils::restore_span($i, re_find_static!($i.fragment.0, $re))
    };
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
                Err::Error(Context::Code(i, kind)) => {
                    Err::Error(Context::Code(span.slice(..i.len()), kind))
                },
                Err::Failure(Context::Code(i, kind)) => {
                    Err::Failure(Context::Code(span.slice(..i.len()), kind))
                },
                Err::Incomplete(needed) => Err::Incomplete(needed),
            }
        })
}
