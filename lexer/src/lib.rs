//! Transformation of NAFI source code into tokens
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]

#[macro_use]
extern crate nom;

mod lexer;
mod tokens;

pub use tokens::Token;

/// Lex NAFI source into its component tokens
pub fn lex(str: &str) -> Result<Vec<Token>, nom::Err<&str>> {
    match lexer::tokens(str) {
        nom::IResult::Done(_, tokens) => Ok(tokens),
        nom::IResult::Error(e) => Err(e),
        nom::IResult::Incomplete(_) => unreachable!("Lexer failed to finish"),
    }
}
