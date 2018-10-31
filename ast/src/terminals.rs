//! The terminal nodes of the Nafi grammar.

use crate::{parser::Rule, Span, Spanned};
use pest_ast::FromPest;
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
#[pest_ast(rule(Rule::Identifier))]
pub struct Identifier<'a> {
    #[pest_ast(outer(with(Into::into)))]
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
#[pest_ast(rule(Rule::IntegerLiteral))]
pub struct IntegerLiteral<'a> {
    #[pest_ast(outer(with(Into::into)))]
    span: Span<'a>,
}
