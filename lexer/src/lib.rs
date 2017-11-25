//! Transformation of NAFI source code into tokens

#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]
#![feature(conservative_impl_trait, const_fn, dotdoteq_in_patterns, never_type, option_filter)]

#[macro_use]
extern crate matches;
extern crate nafi_tokens as tokens;
extern crate nnom;
extern crate str_intern;
extern crate unic_ucd_category;

use nnom::ParseOutput;
use tokens::Token;

mod lexer;

/// Lex NAFI source into its component tokens
pub fn lex(s: &str) -> Vec<Token> {
    let Ok(ParseOutput { output, .. }) = lexer::tokens(s.into());
    output
}
