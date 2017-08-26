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
///         Ok(input.split_at(4))
///     } else {
///         Err(())
///     }
/// }
///
/// assert_eq!(
///     many0(type_keyword)("typetypetipe"),
///     Ok((vec!["type", "type"], "tipe"))
/// )
/// ```
pub fn many0<'a, T: 'a + ?Sized, In, Out, Error, Parser>(
    parser: Parser,
) -> impl Fn(In) -> Result<In, Vec<Out>, !>
where
    In: Slice<'a, T> + Copy,
    Parser: Fn(In) -> Result<In, Out, Error>,
    &'a T: Slice<'a, T>,
{
    move |mut input: In| {
        let mut res = Vec::new();

        while !input.is_empty() {
            match parser(input) {
                Ok((o, i)) => {
                    res.push(o);
                    input = i;
                },
                Err(_) => break,
            }
        }

        Ok((res, input))
    }
}
