//! Tokens of Nafi source

#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
//#![deny(missing_docs, unsafe_code, unused)]
#![feature(match_default_bindings)]

extern crate num_bigint as bigint;

mod literal;

pub use literal::{BigUint, Literal, StringFragments};
