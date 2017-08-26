use std::borrow::Cow;

use num::bigint::BigUint;
use single::Single;

#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    _Unknown,
    Whitespace,

    // == Literals == //
    IntegerLiteral(BigUint),
    StringLiteral(StringFragments),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum StringFragment {
    Str(String),
    InvalidEscape(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringFragments(Vec<StringFragment>);

impl StringFragments {
    pub fn new() -> StringFragments { StringFragments(vec![]) }

    pub fn push(&mut self, char: char) {
        let len = self.0.len();
        if len == 0 {
            self.0.push(StringFragment::Str(char.to_string()));
        } else {
            if let StringFragment::Str(_) = self.0[len - 1] {
                if let StringFragment::Str(ref mut string) = self.0[len - 1] {
                    string.push(char);
                }
            } else {
                self.0.push(StringFragment::Str(char.to_string()));
            }
        }
    }

    pub fn push_str<'a, S: Into<Cow<'a, str>>>(&mut self, str: S) {
        let len = self.0.len();
        if len == 0 {
            self.0.push(StringFragment::Str(str.into().into_owned()));
        } else {
            if let StringFragment::Str(_) = self.0[len - 1] {
                if let StringFragment::Str(ref mut string) = self.0[len - 1] {
                    string.push_str(str.into().to_mut());
                }
            } else {
                self.0.push(StringFragment::Str(str.into().into_owned()))
            }
        }
    }

    pub fn push_invalid_escape<S: Into<String>>(&mut self, escape: S) {
        self.0.push(StringFragment::InvalidEscape(escape.into()))
    }

    pub fn try_into_string(self) -> Option<String> {
        match self.0.into_iter().single() {
            Ok(fragment) => match fragment {
                StringFragment::Str(string) => Some(string),
                _ => None,
            },
            Err(_) => None,
        }
    }
}

impl<'a> From<&'a str> for StringFragments {
    fn from(str: &'a str) -> Self { StringFragments(vec![StringFragment::Str(str.into())]) }
}
