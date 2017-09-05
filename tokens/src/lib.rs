//! Tokens of Nafi source
#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]
#![feature(conservative_impl_trait)]

extern crate num;

use num::bigint::BigUint;
use std::borrow::Cow;

#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    #[doc(hidden)] _Unknown(usize),
    Whitespace(usize),

    // == Literals == //
    IntegerLiteral(usize, BigUint),
    StringLiteral(usize, StringFragments),
}

impl Token {
    /// The start location of this token.
    pub fn position(&self) -> usize {
        match *self {
            Token::_Unknown(pos) |
            Token::Whitespace(pos) |
            Token::IntegerLiteral(pos, _) |
            Token::StringLiteral(pos, _) => pos
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum StringFragment {
    Str(String),
    InvalidEscape(String),
}

/// A String that also remembers invalid escapes inside it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StringFragments {
    fragments: Vec<StringFragment>,
}

impl StringFragments {
    /// Create a new, empty string.
    pub fn new() -> StringFragments { Default::default() }

    /// Push a character onto the end of this string.
    pub fn push(&mut self, char: char) {
        let len = self.fragments.len();
        if len == 0 {
            self.fragments.push(StringFragment::Str(char.to_string()));
        } else {
            if let StringFragment::Str(_) = self.fragments[len - 1] {
                if let StringFragment::Str(ref mut string) = self.fragments[len - 1] {
                    string.push(char);
                }
            } else {
                self.fragments.push(StringFragment::Str(char.to_string()));
            }
        }
    }

    /// Push a string onto the end of this string.
    pub fn push_str<'a, S: Into<Cow<'a, str>>>(&mut self, str: S) {
        let len = self.fragments.len();
        if len == 0 {
            self.fragments
                .push(StringFragment::Str(str.into().into_owned()));
        } else {
            if let StringFragment::Str(_) = self.fragments[len - 1] {
                if let StringFragment::Str(ref mut string) = self.fragments[len - 1] {
                    string.push_str(str.into().as_ref());
                }
            } else {
                self.fragments
                    .push(StringFragment::Str(str.into().into_owned()))
            }
        }
    }

    /// Push an invalid escape onto the end of this string.
    pub fn push_invalid_escape<S: Into<String>>(&mut self, escape: S) {
        self.fragments
            .push(StringFragment::InvalidEscape(escape.into()))
    }

    /// Try to turn this string into a normal string.
    ///
    /// Fails if any invalid escapes are present.
    pub fn try_into_string(self) -> Result<String, InvalidEscapes> {
        if self.fragments.len() == 1 {
            if let StringFragment::Str(_) = self.fragments[0] {
                if let Some(StringFragment::Str(string)) = self.fragments.into_iter().next() {
                    return Ok(string);
                } else {
                    unreachable!()
                }
            }
        }
        return Err(InvalidEscapes(
            self.fragments
                .into_iter()
                .filter_map(|fragment| match fragment {
                    StringFragment::InvalidEscape(escape) => Some(escape),
                    StringFragment::Str(_) => None,
                })
                .collect(),
        ));
    }
}

/// The invalid escapes in a string literal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidEscapes(Vec<String>);

impl InvalidEscapes {
    /// Create an iterator over the invalid escapes.
    ///
    /// You get what was attached after the `\`.
    /// E.g. `\w` gives `w` and `\u{INVALID}` gives `u{INVALID}`
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a str> { self.0.iter().map(String::as_str) }
}

impl<'a> From<&'a str> for StringFragments {
    fn from(str: &'a str) -> Self {
        StringFragments {
            fragments: vec![StringFragment::Str(str.into())],
        }
    }
}
