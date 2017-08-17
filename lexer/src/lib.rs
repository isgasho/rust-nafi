//! Transformation of NAFI source code into tokens
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
// #![deny(missing_docs, unsafe_code, unused)]

#[macro_use]
extern crate nom;

mod lexer;
mod tokens;

//use lexer::token;
pub use tokens::Token;

named! {
    _lex<&str, Vec<Token>>,
    ws!(
        many0!(lexer::token)
    )
}

pub fn lex(str: &str) -> Result<Vec<Token>, String> {
    match _lex(str) {
        nom::IResult::Done(_, tokens) => Ok(tokens),
        nom::IResult::Error(e) => Err(format!("Internal Lexer Error: {}", e)),
        nom::IResult::Incomplete(_) => Err("Failed to lex".into()),
    }
}
