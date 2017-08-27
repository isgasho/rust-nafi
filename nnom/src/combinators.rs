//! Simple combinator fn of other parser fn

mod protected {
    use prelude::{PositionedSlice, PositionedStr};
    use std::ops;

    pub trait Slice<'a, T: 'a + ?Sized>: ops::Deref<Target = T> + Copy
    where
        &'a T: Slice<'a, T>,
    {
        fn is_empty(&self) -> bool;
    }

    impl<'a> Slice<'a, str> for &'a str {
        fn is_empty(&self) -> bool { (self as &str).is_empty() }
    }
    impl<'a> Slice<'a, str> for PositionedStr<'a> {
        fn is_empty(&self) -> bool { (self as &str).is_empty() }
    }
    impl<'a, T: 'a> Slice<'a, [T]> for &'a [T] {
        fn is_empty(&self) -> bool { (self as &[T]).is_empty() }
    }
    impl<'a, T: 'a> Slice<'a, [T]> for PositionedSlice<'a, T> {
        fn is_empty(&self) -> bool { (self as &[T]).is_empty() }
    }
}

use self::protected::Slice;
use result::*;

/// Construct a new parser that matches a subparser zero or more times.
///
/// This combinator swallows any errors from the subparser and returns
/// a vector of outputs excluding the first parse to error.
///
/// # Example
///
/// ```
/// # use nnom::prelude::*;
/// fn type_keyword(input: &str) -> Result<&str, &str, ()> {
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
pub fn many0<'a, T: 'a + ?Sized, In, Out, Error, Parser>(
    parser: Parser,
) -> impl Fn(In) -> Result<In, Vec<Out>, !>
where
    In: Slice<'a, T>,
    Parser: Fn(In) -> Result<In, Out, Error>,
    &'a T: Slice<'a, T>,
{
    move |mut input: In| {
        let mut result = Vec::new();

        while !input.is_empty() {
            match parser(input) {
                Ok(ParseOutput { remaining_input, output }) => {
                    result.push(output);
                    input = remaining_input;
                },
                Err(_) => break,
            }
        }

        Ok(ParseOutput {
            remaining_input: input,
            output: result,
        })
    }
}

/// Run a parser without consuming anything.
pub fn peek<'a, T: 'a + ?Sized, In, Out, Error, Parser>(
    parser: Parser,
) -> impl Fn(In) -> Result<In, Out, Error>
where
    In: Slice<'a, T>,
    Parser: Fn(In) -> Result<In, Out, Error>,
    &'a T: Slice<'a, T>,
{
    move |input: In| {
        parser(input).map(|ParseOutput { output, .. }| {
            ParseOutput { remaining_input: input, output }
        })
    }
}
