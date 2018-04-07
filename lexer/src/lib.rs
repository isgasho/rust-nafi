//! Transformation of NAFI source code into tokens

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, unsafe_code, missing_docs)]
#![warn(edition_2018, rust_2018_idioms)]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;
extern crate nafi_interner as interner;
extern crate nafi_location as location;
extern crate nafi_tokens as tokens;
#[macro_use(position)]
extern crate nom_locate;
extern crate regex;

use interner::StringInterner;
use tokens::Token;

#[cfg_attr(rustfmt, rustfmt_skip)] // nom macros
mod lexer;

type Cursor<'a> = nom_locate::LocatedSpan<&'a str>;
#[allow(non_snake_case)]
fn Position(pos: Cursor) -> location::Position {
    location::Position {
        line: pos.line,
        column: pos.get_utf8_column() as u32,
    }
}

/// Lexer for Nafi source code
#[derive(Debug)]
pub struct Lexer<'i, 'lex> {
    str_pool: &'lex StringInterner,
    source: Cursor<'i>,
}

impl<'i, 'lex> Lexer<'i, 'lex> {
    /// Create a new Lexer for given source with a given string pool
    pub fn new(source: &'i str, pool: &'lex StringInterner) -> Self {
        Lexer {
            str_pool: pool,
            source: Cursor::new(source),
        }
    }

    fn try_next(&mut self) -> Result<Token<'lex>, nom::Err<Cursor<'i>>> {
        lexer::token(self.source, self.str_pool).map(|(i, o)| {
            self.source = i;
            o
        })
    }
}

impl<'i, 'lex> Iterator for Lexer<'i, 'lex> {
    type Item = Token<'lex>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(tok) => Some(tok),
            Err(err) => {
                info!("Lexer stopped at {:?}", err);
                None
            },
        }
    }
}
