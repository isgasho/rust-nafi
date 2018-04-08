//! Transformation of NAFI source code into tokens

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, unsafe_code, missing_docs)]
#![warn(edition_2018, rust_2018_idioms)]

// false positive rust-clippy#2586
#![cfg_attr(feature = "cargo-clippy", allow(regex_macro))]
// Skip formatting this crate due to the large use of nom macros
#![cfg_attr(rustfmt, rustfmt_skip)]
// nom macros don't support pub(crate) syntax
#![allow(unreachable_pub)]
#![allow(unused)]

#[macro_use]
extern crate nom;
#[macro_use(position)]
extern crate nom_locate;

#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;
extern crate regex;

extern crate nafi_location as location;
extern crate nafi_tokens as tokens;

/// A cursor into the source code.
type Cursor<'a> = nom_locate::LocatedSpan<nom::types::CompleteStr<'a>>;

/// Create a cursor out of source.
///
/// # Safety
///
/// `source` must be a string reference at least `offset` bytes past the start of the backing slice.
#[allow(non_snake_case, unsafe_code)]
unsafe fn Cursor(source: &str, line: u32, offset: usize) -> Cursor {
    nom_locate::LocatedSpan {
        offset, line,
        fragment: nom::types::CompleteStr(source),
    }
}

/// Create a span spanning between two cursors.
#[allow(non_snake_case)]
fn Span(start: Cursor, stop: Cursor) -> location::Span {
    location::Span {
        start: location::Position {
            line: start.line,
            column: start.get_utf8_column() as u32,
        },
        stop: location::Position {
            line: stop.line,
            column: stop.get_utf8_column() as u32,
        },
    }
}

#[macro_use]
mod utils;
mod code;
mod string;

/// Lexer to transform Nafi source code into tokens
#[derive(Copy, Clone, Debug)]
pub struct Lexer<'a> {
    source: Cursor<'a>,
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(source: &'a str) -> Self {
        Lexer::new(source)
    }
}

impl<'a> Lexer<'a> {
    /// Create a new lexer from the given source.
    pub fn new(source: &'a str) -> Self {
        Lexer::new_on_line(source, 1)
    }

    /// Create a new lexer that starts on the given line.
    ///
    /// The given source must be the start of the line, or
    /// span information will be incorrect for the first line.
    #[allow(unsafe_code)]
    pub fn new_on_line(source: &'a str, line: u32) -> Self {
        Lexer {
            source: unsafe { Cursor(source, line, 0) }
        }
    }
}

impl<'a> Lexer<'a> {
    /// Produce the next token in CODE mode
    pub fn next_code(&mut self) -> Option<tokens::code::Token<'a>> {
        use nom::InputLength;
        if self.source.input_len() == 0 {
            return None;
        }
        if let Ok((i, o)) = alt!(self.source,
            call!(code::comment) |
            call!(code::whitespace) |
            call!(code::literal_string_start) |
            call!(code::literal_integer) |
            call!(code::symbol) |
            call!(code::identifier)
        ) {
            self.source = i;
            Some(o)
        } else {
            error!("CODE mode Lexer terminated early with remaining input `{:?}`", self.source);
            None
        }
    }

    /// Produce the next token in STRING mode
    pub fn next_string(&mut self) -> Option<tokens::string::Token<'a>> {
        use nom::InputLength;
        if self.source.input_len() == 0 {
            return None;
        }
        if let Ok((i, o)) = alt!(self.source,
            call!(string::end) |
            call!(string::interpolation_start) |
            call!(string::escaped) |
            call!(string::text)
        ) {
            self.source = i;
            Some(o)
        } else {
            error!("STRING mode Lexer terminated early with remaining input `{:?}`", self.source);
            None
        }
    }
}
