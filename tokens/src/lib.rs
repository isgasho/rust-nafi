//! Tokens of Nafi source

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, missing_docs, unsafe_code, unused)]
#![feature(conservative_impl_trait, match_default_bindings)]

#[macro_use]
extern crate lazy_static;
extern crate num_bigint as bigint;
extern crate string_interner;

mod literal;
mod symbol;

pub use literal::{BigUint, Literal, StringFragments};
pub use symbol::Symbol;

use std::fmt;
use std::sync::Mutex;

/// A token in the source code. A simple atomic unit.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Token {
    Identifier(usize, Identifier),
    Keyword(usize, Keyword),
    Symbol(usize, Symbol),
    Literal(usize, Literal),
    Whitespace(usize),
    #[doc(hidden)] _Unknown(usize, char),
}

impl Token {
    /// The start position of this token.
    pub fn position(&self) -> usize {
        match *self {
            Token::Identifier(pos, _)
            | Token::Keyword(pos, _)
            | Token::Symbol(pos, _)
            | Token::Literal(pos, _)
            | Token::Whitespace(pos)
            | Token::_Unknown(pos, _) => pos,
        }
    }
}

lazy_static! {
    static ref CACHE: Mutex<string_interner::DefaultStringInterner> = Default::default();
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct Identifier(usize);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cache = CACHE.lock().expect("Poisoned Mutex");
        cache.resolve(self.0).unwrap().fmt(f)
    }
}

impl<'a> From<&'a str> for Identifier {
    fn from(s: &str) -> Identifier {
        Identifier(CACHE.lock().expect("Poisoned Mutex").get_or_intern(s))
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
