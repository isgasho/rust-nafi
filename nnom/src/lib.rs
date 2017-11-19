//! A minimal nom-like parser combinator library.
//!
//! Inspired by [`nom`](https://crates.io/crates/nom) and [`synom`](https://crates.io/crates/synom),
//! `nnom` is a custom-built parser combinator built without macros utilizing unstable Rust features
//! for use in developing the language Nafi.
//!
//! By its very nature `nnom` is a lot more unstable than `synom` or `nom`. It is built using
//! cutting-edge Rust features when possible in order to give a biased opinion of the cleanest
//! use-case without macros. Macros mess up intellisense. This may cause a slight performance hit,
//! but the purpose of this library is to enable quick prototyping, not the most efficient product.

#![forbid(bad_style, missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(missing_docs, unsafe_code, unused)]
#![feature(conservative_impl_trait, never_type)]

pub mod combinators;
mod result;
pub mod slice;

pub use result::{ParseOutput, ParseResult};
