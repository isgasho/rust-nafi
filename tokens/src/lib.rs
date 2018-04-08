//! Tokens of Nafi source

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, missing_docs, unsafe_code, unused)]
#![warn(unreachable_pub)]

#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate smart_default;
extern crate nafi_location as location;

/// A source of tokens (e.g. a lexer)
trait Lexer {
    type Error;
    fn next_code(source: &str) -> Result<code::Token, Self::Error>;
    fn next_string(source: &str) -> Result<string::Token, Self::Error>;
}

pub mod code;
pub mod string;
