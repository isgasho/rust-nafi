//! Abstract Syntax Tree of Nafi source

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, unsafe_code, missing_docs)]
#![warn(edition_2018, rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

extern crate nafi_location as location;
extern crate nafi_tokens as tokens;
extern crate num_bigint as bigint;

#[allow(missing_docs)]
pub mod expression;
pub use expression::Expression;

#[allow(missing_docs)]
pub mod statement;
pub use statement::Statement;

#[allow(missing_docs)]
mod misc;
pub use misc::*;
