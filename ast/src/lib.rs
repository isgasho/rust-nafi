//! # Abstract Syntax Tree for the Nafi programming language.
//!
//! This is the second representation of source code as it gets processed.
//!
//! The first is the Lossless Syntax Tree, which is what most external syntax consumers should use.
//! It is not only more fault-tolerant, but also allows for manipulation and can be turned back
//! into code to be saved to the filesystem. The LST is also the level at which syntax extensions
//! (are planned to) work. (Note: This level is currently not present; instead, the pest Parse Tree
//! fills this role instead. Ultimately an IDE-ready libsyntax2-style LST will be used here.)
//!
//! The second is the Abstract Syntax Tree. This is the grammar of Nafi stripped to its bare parts:
//! whitespace, symbols, any trace of how it was structured in text is gone. This is still a thin
//! transformation; the information in the AST is still purely syntactical, it's just represented
//! in an owned, infallible tree. (Note: This level may disappear as a zero-cost wrapper around
//! the LST rather than a separate representation in the future.)
//!
//! The third is the High-level Intermediate Representation. This is the level that de-sugars and
//! makes everything explicit. The HIR is entirely divorced from the text representation of the
//! language, and is instead the result of interpreting that definition. To draw a parallel to the
//! written format, HIR is fully elaborated: all optional annotations are present, all paths are
//! absolute and concrete, and all syntax sugar and implicit meaning has been made explicit.
//!
//! # Notes
//!
//! The public dependency on `pest` is an implementation detail, and trait implementations of `pest`
//! traits are only public due to a limitation of Rust. They may be hidden or removed in the future.
//!
//! If a `pest` type appears in public API that is _not_ via a trait implementation, that is part of
//! the API and will incur a breaking change to remove. (This is a bug if so; please report it.)
//!
//! The rough plan is that the `pest` parser will remain as a reference parser and a way to test the
//! grammar specified on the AST types, but the official parser will migrate to a lossless IDE-ready
//! syntax tree in the future, as tooling making the development of such a parser becomes available.
//!
//! # Grammar
//!
//! The following general productions are used throughout the rest of the specific grammars.
//!
//! ```pest,no_run
//! Separated(Rule, Separator) =
//!    _{ Rule
//!     ~ ( Separator
//!       ~ Rule
//!       )*
//!     }
//!
//! CommaSeparated(Rule) =
//!    _{ Separated(Rule, ",")
//!     ~ ","?
//!     }
//!
//! Keyword(Word) =
//!   _${ Word
//!     ~ !XID_CONTINUE
//!     }
//!
//! TypeAscription =
//!    _{ ":"
//!     ~ Path
//!     }
//!
//! __incomplete = // unmatchable, serves to mark incomplete choices
//!    _{ !ANY
//!     ~ ANY
//!     }
//!
//! WHITESPACE =
//!    _{ WHITE_SPACE
//!     }
//!
//! COMMENT =
//!    _{ line_comment
//!     | block_comment
//!     }
//!
//! line_comment =
//!    _{ "//"
//!     ~ ( !NEWLINE
//!       ~ ANY
//!       )*
//!     }
//!
//! block_comment =
//!    _{ "/*"
//!     ~ ( block_comment
//!       | ( !"*/"
//!         ~ ANY
//!         )
//!       )*
//!     ~ "*/"
//!     }
//! ```

#![warn(missing_docs)]

pub mod containers;
pub mod functions;
pub mod paths;
pub mod terminals;

#[doc(hidden)]
pub mod parser {
    use pest_derive::Parser;
    #[derive(Parser)]
    #[grammar = "grammar.pest"]
    pub struct Parser;
}

mod span;
pub use self::span::{Span, Spanned};
