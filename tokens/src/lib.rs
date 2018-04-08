//! Tokens of Nafi source

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, missing_docs, unsafe_code, unused)]
#![warn(unreachable_pub)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde_derive;
extern crate nafi_location as location;

pub mod code;
pub mod string;
