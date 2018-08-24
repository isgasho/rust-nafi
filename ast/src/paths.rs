//! Paths are used to uniquely refer to one type or place in the program.

use crate::terminals::Identifier;
use crate::{Span, Spanned};
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
#[derive(Serialize, Spanned)]
pub struct Path<'a> {
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
#[derive(Serialize, Spanned)]
pub struct PathSegment<'a> {
    span: Span<'a>,
    name: Identifier<'a>,
    arguments: Vec</*Expression<'a>*/ ()>,
}
