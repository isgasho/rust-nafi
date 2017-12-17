//! Transformation of NAFI source code into tokens

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, missing_docs, unsafe_code, unused)]
#![feature(dotdoteq_in_patterns)]
//#![feature(conservative_impl_trait, const_fn, dotdoteq_in_patterns, never_type, option_filter)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate lazy_static;
extern crate nafi_tokens as tokens;
extern crate regex;
extern crate single;

use tokens::Token;

#[macro_use]
mod span;
mod lexer;

type Span<'a> = span::LocatedSpan<&'a str>;

/// Lex NAFI source into its component tokens
pub fn lex(s: &str) -> Vec<Token> {
    match lexer::tokens(Span::new(s)) {
        Ok((_i, o)) => o,
        Err(ctx) => unreachable!("Lexer failed with context {:?}", ctx),
    }
}
