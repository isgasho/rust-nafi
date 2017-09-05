//! Tokens of Nafi source
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]
#![feature(conservative_impl_trait)]

extern crate num;

mod symbol;
mod literal;

pub use literal::{BigUint, Literal, StringFragments};
pub use symbol::Symbol;

/// A token in the source code. Simply chunking the source into units to then parse.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Token {
    #[doc(hidden)] _Unknown(usize),
    Whitespace(usize),
    Symbol(usize, Symbol),
    Literal(usize, Literal),
    Keyword(usize, Keyword),
    Identifier(usize, String),
}

impl Token {
    /// The start location of this token.
    pub fn position(&self) -> usize {
        match *self {
            Token::_Unknown(pos) |
            Token::Whitespace(pos) |
            Token::Symbol(pos, _) |
            Token::Literal(pos, _) |
            Token::Keyword(pos, _) |
            Token::Identifier(pos, _) => pos,
        }
    }
}

/// A reserved identifier-like in the source code.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Keyword {
    Let,
    Mutable,
    If,
    Else,
}
