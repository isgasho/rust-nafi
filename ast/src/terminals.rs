//! The terminal nodes of the Nafi grammar.

use crate::{parser::Rule, Span, Spanned};
use pest_deconstruct::FromPest;
use serde::Serialize;

/// A name referring to some place that information can be stored.
///
/// # Grammar
///
/// ```pest,no_run
/// // UAX31-R1 Default Identifier grammar
/// Identifier = @{ XID_START ~ XID_CONTINUE* }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned, FromPest)]
#[pest(rule = "Rule::Identifier")]
pub struct Identifier<'a> {
    span: Span<'a>,
}

/// An integer in the source code.
///
/// # Grammar
///
/// ```pest,no_run
/// IntegerLiteral = @{ ASCII_DIGIT+ }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned, FromPest)]
#[pest(rule = "Rule::IntegerLiteral")]
pub struct IntegerLiteral<'a> {
    span: Span<'a>,
}
