//! Simple combinator fn of other parser fn

mod protected {
    use prelude::{PositionedSlice, PositionedStr};
    use std::ops;

    pub trait Slice<'a, T: 'a + ?Sized>: ops::Deref<Target = T>
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
use Result;
use std::fmt;

/// Construct a new parser that matches a subparser zero or more times.
///
/// # Example
///
/// ```
/// # use nnom::prelude::*;
/// fn type_keyword(input: &str) -> Result<&str> {
///     if input.starts_with("type") {
///         Result::Done(&input[4..], &input[..4])
///     } else {
///         Result::Pass
///     }
/// }
///
/// assert_eq!(
///     many0(type_keyword)("typetypetipe"),
///     Result::Done("tipe", vec!["type", "type"])
/// )
/// ```
pub fn many0<'a, T: 'a + ?Sized, In, Out, Failure, Parser>(
    parser: Parser,
) -> impl Fn(In) -> Result<In, Vec<Out>, Failure>
where
    In: Slice<'a, T> + Copy,
    Parser: Fn(In) -> Result<In, Out, Failure>,
    Failure: fmt::Display,
    &'a T: Slice<'a, T>,
{
    move |mut input: In| {
        let mut res = Vec::new();

        while !input.is_empty() {
            match parser(input) {
                Result::Done(i, o) => {
                    res.push(o);
                    input = i;
                },
                Result::Abort(failure) => return Result::Abort(failure),
                Result::Pass => break,
            }
        }

        Result::Done(input, res)
    }
}
