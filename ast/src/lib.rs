//! Abstract Syntax Tree of Nafi source

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, unsafe_code, missing_docs)]
#![warn(edition_2018, rust_2018_idioms)]

extern crate nafi_tokens as tokens;
extern crate num_bigint as bigint;

mod expression;

pub use expression::*;
