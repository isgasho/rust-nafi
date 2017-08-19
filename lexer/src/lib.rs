//! Transformation of NAFI source code into tokens
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]

#[macro_use]
extern crate nom;
extern crate num;
extern crate single;

mod lexer;
mod tokens;

pub use tokens::Token;

/// Lex NAFI source into its component tokens
pub fn lex(str: &str) -> Vec<Token> {
    match lexer::tokens(str) {
        nom::IResult::Done(_, tokens) => tokens,
        nom::IResult::Error(e) => unreachable!("Lexer failed to recover from error {}", e),
        nom::IResult::Incomplete(_) => unreachable!("Lexer failed to finish"),
    }
}
