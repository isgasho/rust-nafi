pub use bigint::BigUint;
use std::borrow::Cow;

/// A literal in the source code, e.g. a string or number.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    Integer(BigUint),
    String(StringFragments),
}

impl From<BigUint> for Literal {
    fn from(uint: BigUint) -> Self { Literal::Integer(uint) }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum StringFragment {
    String(String),
    InvalidEscape(String),
}

impl<S: Into<String>> From<S> for StringFragment {
    fn from(s: S) -> Self { StringFragment::String(s.into()) }
}

/// A String that also remembers invalid escapes inside it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StringFragments {
    fragments: Vec<StringFragment>,
}

impl<S: Into<String>> From<S> for StringFragments {
    fn from(s: S) -> Self {
        StringFragments {
            fragments: vec![s.into().into()],
        }
    }
}

impl StringFragments {
    /// Create a new, empty string.
    pub fn new() -> StringFragments { Default::default() }

    /// Push a character onto the end of this string.
    pub fn push_char(&mut self, ch: char) {
        match self.fragments.last_mut() {
            Some(StringFragment::String(mut string)) => string.push(ch),
            _ => self.fragments.push(ch.to_string().into()),
        }
    }

    /// Push a string onto the end of this string.
    pub fn push_string<'a, S: Into<Cow<'a, str>>>(&mut self, s: S) {
        match self.fragments.last_mut() {
            // removing the `ref` from the following line causes an ICE for some reason
            Some(StringFragment::String(ref mut string)) => string.push_str(s.into().as_ref()),
            _ => self.fragments.push(s.into().into_owned().into()),
        }
    }

    /// Push an invalid escape onto the end of this string.
    pub fn push_invalid_escape<S: Into<String>>(&mut self, s: S) {
        self.fragments.push(StringFragment::InvalidEscape(s.into()))
    }
}
