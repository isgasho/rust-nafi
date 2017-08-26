//! A minimal presentation of a nom-like parser combinator framework.
//!
//! Heavily inspired by [`synom`](https://dtolnay.github.io/syn/synom/),
//! but using only the parts I need.
//!
//! At the most basic, a parser fn takes the form
//! `fn(input: Slice) -> nnom::Result<Slice, Slice, Error>` for some `Slice` and `Error`.
//! It then returns `Ok((parsed, remaining_input))` or the appropriate error.
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]
#![recursion_limit = "1024"]
#![feature(conservative_impl_trait, never_type)]

pub mod combinators;
pub mod slice;

/// Standard result type for parsing.
///
/// Ok is a tuple of the parsed output and the remaining unused input.
pub type Result<In, Out, Error> = ::std::result::Result<(Out, In), Error>;

#[allow(missing_docs)]
pub mod prelude {
    pub use Result;
    pub use combinators::many0;
    pub use slice::{PositionedSlice, PositionedStr};
}
