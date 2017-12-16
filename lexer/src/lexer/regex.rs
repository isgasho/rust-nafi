use Span;
use nom::{IResult, Slice};

pub fn restore_span<'a>(span: Span<'a>, result: IResult<&str, &str>) -> IResult<Span<'a>, Span<'a>> {
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
