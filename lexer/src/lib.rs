//! Transformation of NAFI source code into tokens
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]
#![feature(never_type)]

#[macro_use]
extern crate error_chain;

extern crate nafi_tokens as tokens;
extern crate nnom;
extern crate num;

/// `error-chain` error types
pub mod error {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt;
        }
        errors {
            /// A parser fragment didn't match the given input.
            NoMatch(pos: usize, parser: &'static str) {
                description("parser fragment did not match"),
                display("parser fragment {} did not match starting at byte {}", parser, pos),
            }
        }
    }
}

mod lexer;

use nnom::prelude::ParseOutput;
pub use tokens::Token;

/// Lex NAFI source into its component tokens
pub fn lex(str: &str) -> Vec<Token> {
    let Ok(ParseOutput { output, .. }) = lexer::tokens(str.into());
    output
}
