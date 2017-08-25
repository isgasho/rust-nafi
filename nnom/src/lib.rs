//! A minimal presentation of a nom-like parser combinator framework.
//!
//! Heavily inspired by [`synom`](https://dtolnay.github.io/syn/synom/),
//! but only the parts I need and a bit more space to support decent error handling.
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]

mod macros;
mod result;
pub mod slice;

pub use result::Result;
pub use slice::PositionedStr;
pub use slice::PositionedSlice;
pub use slice::PositionedIndex;
