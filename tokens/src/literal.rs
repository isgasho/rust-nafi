use Token;
pub use bigint::BigUint;
use std::borrow::Cow;

/// A literal in the source code, e.g. a string or number.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
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
    Interpolation(Vec<Token>),
}

impl From<StringFragments> for Literal {
    fn from(fragments: StringFragments) -> Self { Literal::String(fragments) }
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
            Some(StringFragment::String(string)) => string.push(ch),
            _ => self.fragments.push(ch.to_string().into()),
        }
    }

    /// Push a string onto the end of this string.
    pub fn push_string<'a, S: Into<Cow<'a, str>>>(&mut self, s: S) {
        match self.fragments.last_mut() {
            Some(StringFragment::String(string)) => string.push_str(s.into().as_ref()),
            _ => self.fragments.push(s.into().into_owned().into()),
        }
    }

    /// Push an invalid escape onto the end of this string.
    pub fn push_invalid_escape<S: Into<String>>(&mut self, s: S) {
        self.fragments.push(StringFragment::InvalidEscape(s.into()))
    }

    /// Push the tokens inside string interpolation onto the end of this string.
    pub fn push_interpolation(&mut self, mut t: Vec<Token>) {
        match self.fragments.last_mut() {
            Some(StringFragment::Interpolation(tokens)) => tokens.append(&mut t),
            _ => self.fragments.push(StringFragment::Interpolation(t)),
        }
    }

    /// Try to turn this string into a normal string.
    ///
    /// Fails if any invalid escapes are present.
    pub fn try_into_string(self) -> Result<String, InvalidEscapes> {
        if self.fragments.len() == 1 {
            if let StringFragment::String(_) = self.fragments[0] {
                if let Some(StringFragment::String(string)) = self.fragments.into_iter().next() {
                    return Ok(string);
                } else {
                    unreachable!()
                }
            }
        }
        Err(InvalidEscapes(
            self.fragments
                .into_iter()
                .filter_map(|fragment| match fragment {
                    StringFragment::InvalidEscape(escape) => Some(escape),
                    _ => None,
                })
                .collect(),
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Fail)]
#[fail(display = "StringFragments contained (an) invalid escape(s)")]
pub struct InvalidEscapes(Vec<String>);

impl InvalidEscapes {
    /// Create an iterator over the invalid escapes.
    ///
    /// You get what was attached after the `\`.
    /// E.g. `\w` gives `w` and `\u{INVALID}` gives `u{INVALID}`
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a str> { self.0.iter().map(String::as_str) }
}
