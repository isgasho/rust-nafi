//! A very simplistic string interning interface based around giving out `&str` references
//! rather than some placeholder symbol. This means that strings can be interned in systems
//! based around `&str` without rewriting to support a new `Symbol` type.
//!
//! The typical use case for something like this is text processing chunks, where chunks are very
//! likely to be repeated. For example, when parsing source code, identifiers are likely to come up
//! multiple times. Rather than have a `Token::Identifier(String)` and allocate every occurrence of
//! those identifiers separately, interners allow you to store `Token::Identifier(Symbol)`, and
//! compare identifier equality by the interned symbol.
//!
//! This crate provides the option of using the `&str` directly as the `Symbol` type rather than
//! have another layer of indirection to getting the backing slice. This is good for overlaying
//! on top of an existing system that doesn't need to know about the interning going on behind the
//! scenes. However, this means that comparison of interned strings is still `O(len)` when it could
//! be a simple pointer compare, and interned symbols cannot be persisted across serialization.
//!
//! If it doesn't make sense for you to give up the benefits of using dedicated symbols in order to
//! get the niche benefit of just using `&str`, you should not use this crate. Consider instead
//! [string-interner](https://crates.io/crates/string-interner), which is based off of the Rust
//! compiler's string interner.

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, unsafe_code, missing_docs)]
#![warn(edition_2018, rust_2018_idioms)]

#[macro_use]
extern crate serde_derive;

use std::{mem, collections::{HashSet, hash_map::RandomState}, hash::BuildHasher, sync::RwLock};

// The `StringInterner` loans out string references with the same lifetime as its own.
// This guarantees that for as long as the interner is alive, so will the loan.
// Because a `String`'s data lives on the heap and we don't mutate them,
// their data will live until they are freed, and will not move, even as our set grows.
// They will not be freed until we are, as we are an append-only collection of `String`s.

/// A string interner based on a `HashSet`. See the crate-level docs for more.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct StringInterner<H: BuildHasher = RandomState> {
    #[serde(bound(deserialize = "H: Default"))] // HashSet: Serialize
    arena: RwLock<HashSet<Box<str>, H>>,
}

// Cannot be derived with the BuildHasher generic
impl Default for StringInterner {
    fn default() -> Self {
        StringInterner {
            arena: RwLock::default(),
        }
    }
}

#[inline(always)]
#[cfg_attr(feature = "cargo-clippy", allow(inline_always))]
fn coerce<T>(t: T) -> T { t }

#[allow(unsafe_code)]
/// The string interner interface
impl<H: BuildHasher> StringInterner<H> {
    /// Get an interned string slice out of this interner, or insert if it doesn't exist.
    /// Takes borrowed or owned strings. If given a new borrowed string, it will be boxed
    /// and saved into the interner. If given an owned string, no new allocation will
    /// happen for the string.
    ///
    /// Note that the interner may need to reallocate to make space for the new reference,
    /// just the same as a `Vec<String>` would. This cost is amortized to `O(1)` as it is
    /// in other standard library collections.
    ///
    /// If you have an owned string and no longer need the ownership, pass it in directly.
    /// Otherwise, just pass in a string slice.
    ///
    /// See `get` for more about the interned `&str`.
    pub fn get_or_insert<'a, S>(&'a self, s: S) -> &'a str
    where
        S: AsRef<str> + Into<Box<str>>,
    {
        if let Some(s) = self.get(s.as_ref()) {
            return s;
        } else {
            let mut arena = self.arena.write().unwrap();
            if let Some(s) = arena.get(s.as_ref()) {
                return unsafe { mem::transmute(coerce::<&str>(s)) };
            } // cannot else because `arena` would still be borrowed
            let boxed_s: Box<str> = s.into();
            // Get the reference to loan out _after_ boxing up our data
            let s_ref: &'a str = unsafe { mem::transmute(coerce::<&str>(&boxed_s)) };
            arena.insert(boxed_s);
            s_ref
        }
    }

    /// Get an interned string slice out of this interner.
    ///
    /// The returned string slice is `&'a str`. This guarantees that the returned slice
    /// will live at least as long as and no longer than this interner does. All strings
    /// in the interner are never mutated, so the heap-allocated string slice is never
    /// going to move, which makes loaning these references out sound.
    pub fn get<'a>(&'a self, s: &str) -> Option<&'a str> {
        self.arena
            .read()
            .unwrap()
            .get(s)
            .map(|s| unsafe { mem::transmute(coerce::<&str>(s)) })
    }
}

/// Constructors
impl StringInterner<RandomState> {
    /// Create an empty string interner.
    ///
    /// The backing set is initially created with a capacity of 0,
    /// so it will not allocate until it is first inserted into.
    pub fn new() -> Self {
        StringInterner {
            arena: RwLock::new(HashSet::new()),
        }
    }

    /// Create an empty string interner with the specified capacity.
    ///
    /// The interner will be able to hold at least `capacity` strings without reallocating.
    /// If `capacity` is 0, the interner will not initially allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        StringInterner {
            arena: RwLock::new(HashSet::with_capacity(capacity)),
        }
    }
}

/// Constructors to control the backing `HashSet`'s hash function
impl<H: BuildHasher> StringInterner<H> {
    /// Create an empty string interner which will use the given hasher to hash the strings.
    ///
    /// The string interner is also created with the default capacity.
    pub fn with_hasher(hasher: H) -> Self {
        StringInterner {
            arena: RwLock::new(HashSet::with_hasher(hasher)),
        }
    }

    /// Create an empty interner with the specified capacity, using `hasher` to hash the strings.
    ///
    /// The interner will be able to hold at least `capacity` strings without reallocating.
    /// If `capacity` is 0, the interner will not initially allocate.
    pub fn with_capacity_and_hasher(capacity: usize, hasher: H) -> Self {
        StringInterner {
            arena: RwLock::new(HashSet::with_capacity_and_hasher(capacity, hasher)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage() {
        // Create the interner
        let interner = StringInterner::default();

        // Intern some strings
        let a1 = interner.get_or_insert(Box::<str>::from("a"));
        let b1 = interner.get_or_insert(String::from("b"));
        let c1 = interner.get_or_insert("c");

        // Get the interned strings
        let a2 = interner.get_or_insert("a");
        let b2 = interner.get_or_insert("b");
        let c2 = interner.get_or_insert("c");

        let a3 = interner.get("a").unwrap();
        let b3 = interner.get("b").unwrap();
        let c3 = interner.get("c").unwrap();

        // The same strings better be the same pointers or it's broken
        assert_eq!(a1.as_ptr(), a2.as_ptr());
        assert_eq!(a2.as_ptr(), a3.as_ptr());
        assert_eq!(b1.as_ptr(), b2.as_ptr());
        assert_eq!(b2.as_ptr(), b3.as_ptr());
        assert_eq!(c1.as_ptr(), c2.as_ptr());
        assert_eq!(c2.as_ptr(), c3.as_ptr());
    }
}
