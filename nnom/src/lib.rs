//! A minimal presentation of a nom-like parser combinator framework.
//!
//! Heavily inspired by [`synom`](https://dtolnay.github.io/syn/synom/),
//! but only the parts I need and a bit more space to support decent error handling.
//!
//! At the most basic, a parser fn takes the form `fn(input: Slice) -> nnom::Result<Slice>`
//! for some `Slice`, usually `nnom::slice::PositionedIndex`. It then returns
//! `nnom::Result::Done(leftover_input, matched_input)` or the appropriate failure case.
//! To that end, parsers are usually bound to work only on sliceable (`&slice[..]`) types.
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]
#![feature(conservative_impl_trait)]

pub mod combinators;
mod result;
pub mod slice;

pub use result::Result;

#[allow(missing_docs)]
pub mod prelude {
    pub use Result;
    pub use combinators::many0;
    pub use slice::{PositionedSlice, PositionedStr};
}
