//! Tokens of Nafi source

#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]
#![feature(conservative_impl_trait, match_default_bindings)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate num_bigint as bigint;

use std::rc::Rc;

mod literal;
mod symbol;

pub use literal::{BigUint, Literal, StringFragments};
pub use symbol::Symbol;

/// A token in the source code. A simple atomic unit.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Token {
    Identifier(usize, Rc<str>),
    Keyword(usize, Keyword),
    Symbol(usize, Symbol),
    Literal(usize, Literal),
    Whitespace(usize),
    #[doc(hidden)] _Unknown(usize),
}

impl Token {
    /// The start position of this token.
    pub fn position(&self) -> usize {
        match *self {
            Token::Identifier(pos, _) |
            Token::Keyword(pos, _) |
            Token::Symbol(pos, _) |
            Token::Literal(pos, _) |
            Token::Whitespace(pos) |
            Token::_Unknown(pos) => pos,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Keyword {
    Let,
    Mutable,
    If,
    Else,
}
