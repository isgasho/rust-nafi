//! Simple combinator fn of other parser fn

use result::{ParseOutput, ParseResult};

mod protected {
    use slice::{PositionedSlice, PositionedStr};

    pub trait Slice<'a, T: 'a + ?Sized>: Copy
    where
        &'a T: Slice<'a, T>,
    {
        fn is_empty(&self) -> bool;
    }

    impl<'a> Slice<'a, str> for &'a str {
        fn is_empty(&self) -> bool { (self as &str).is_empty() }
    }

    impl<'a> Slice<'a, str> for PositionedStr<'a> {
        fn is_empty(&self) -> bool { self.raw_slice().is_empty() }
    }

    impl<'a, T: 'a> Slice<'a, [T]> for &'a [T] {
        fn is_empty(&self) -> bool { (self as &[T]).is_empty() }
    }

    impl<'a, T: 'a> Slice<'a, [T]> for PositionedSlice<'a, T> {
        fn is_empty(&self) -> bool { self.raw_slice().is_empty() }
    }
}

use self::protected::Slice;

/// Construct a new parser that matches a subparser zero or more times.
///
/// This combinator swallows any errors from the subparser and returns
/// a vector of outputs excluding the first parse to error.
///
/// # Example
///
/// ```
/// use nnom::{ParseOutput, ParseResult};
/// use nnom::combinators::many0;
///
/// fn type_keyword(input: &str) -> ParseResult<&str, &str, ()> {
///     if input.starts_with("type") {
///         let (output, remaining_input) = input.split_at(4);
///         Ok(ParseOutput { remaining_input, output })
///     } else {
///         Err(())
///     }
/// }
///
/// assert_eq!(
///     many0(type_keyword)("typetype_other"),
///     Ok(ParseOutput {
///         remaining_input: "_other",
///         output: vec!["type", "type"],
///     })
/// )
/// ```
pub fn many0<'a, T: 'a + ?Sized, In, Out, Error>(
    parser: impl Fn(In) -> ParseResult<In, Out, Error>,
) -> impl Fn(In) -> ParseResult<In, Vec<Out>, !>
where
    In: Slice<'a, T>,
    &'a T: Slice<'a, T>,
{
    move |mut input: In| {
        let mut result = Vec::new();

        while let Ok(parse_output) = parser(input) {
            result.push(parse_output.output);
            input = parse_output.remaining_input;
        }

        Ok(ParseOutput {
            remaining_input: input,
            output: result,
        })
    }
}

/// Run a parser fragment without consuming anything.
pub fn peek<'a, T: 'a + ?Sized, In, Out, Error>(
    parser: impl Fn(In) -> ParseResult<In, Out, Error>,
) -> impl Fn(In) -> ParseResult<In, Out, Error>
where
    In: Slice<'a, T>,
    &'a T: Slice<'a, T>,
{
    move |input: In| {
        parser(input).map(|ParseOutput { output, .. }| ParseOutput {
            remaining_input: input,
            output,
        })
    }
}

/// Construct a new parser that matches a subparser zero or one times.
pub fn optional<'a, T: 'a + ?Sized, In, Out, Error>(
    parser: impl Fn(In) -> ParseResult<In, Out, Error>,
) -> impl Fn(In) -> ParseResult<In, Option<Out>, !>
where
    In: Slice<'a, T>,
    &'a T: Slice<'a, T>,
{
    move |input: In| {
        if let Ok(parse_output) = parser(input) {
            Ok(ParseOutput {
                remaining_input: parse_output.remaining_input,
                output: Some(parse_output.output),
            })
        } else {
            Ok(ParseOutput {
                remaining_input: input,
                output: None,
            })
        }
    }
}
