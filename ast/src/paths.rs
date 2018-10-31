//! Paths are used to uniquely refer to one type or place in the program.

use crate::{containers::Expression, parser::Rule, terminals::Identifier, Span, Spanned};
use pest_ast::FromPest;
use serde::Serialize;

/// A Path is a sequence of `::`-delimited segments referring to a type or data place.
///
/// # Grammar
///
/// ```pest,no_run
/// Path =
///     { "::"?
///     ~ Separated(PathSegment, "::")
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned, FromPest)]
#[pest_ast(rule(Rule::Path))]
pub struct Path<'a> {
    #[pest_ast(outer(with(Into::into)))]
    span: Span<'a>,
    segments: Vec<PathSegment<'a>>,
}

/// A Path Segment uniquely refers to a type, data place, or module.
/// It takes arguments for parameterized types and modules.
///
/// # Grammar
///
/// ```pest,no_run
/// PathSegment =
///     { Identifier
///     ~ ( "["
///       ~ CommaSeparated(Expression)
///       ~ "]"
///       )?
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned, FromPest)]
#[pest_ast(rule(Rule::PathSegment))]
pub struct PathSegment<'a> {
    #[pest_ast(outer(with(Into::into)))]
    span: Span<'a>,
    name: Identifier<'a>,
    arguments: Vec<Expression<'a>>,
}
