//! Abstract Syntax Tree of Nafi source

#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]

extern crate nafi_tokens as tokens;

mod expression;

pub use expression::*;
