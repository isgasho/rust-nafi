//! A minimal presentation of a nom-like parser combinator framework.
//!
//! Heavily inspired by [`synom`](https://dtolnay.github.io/syn/synom/),
//! but using only the parts I need.
//!
//! # Usage
//!
//! A parser fragment takes the form `fn(input: In) -> Result<In, Out, Error>`.
//! On a successful parse, a `ParseResult` is returned, which contains the unused slice of the
//! input and the parsed value. On a failed parse, a hopefully descriptive error is provided.
//!
//! Here `In` stands in for some slice that you work on. `Out` is commonly the matched slice,
//! or some transformed form, such as a token or AST. This crate does not prescribe error handling,
//! but suggests that using `error-chain` is beneficial for passing errors up the parser chain.
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]
#![feature(conservative_impl_trait, never_type)]

pub mod combinators;
pub mod result;
pub mod slice;

#[allow(missing_docs)]
pub mod prelude {
    pub use combinators::many0;
    pub use result::{ParseOutput, Result};
    pub use slice::{PositionedSlice, PositionedStr};
}
