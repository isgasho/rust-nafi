//! Containers (for lack of a better module for them) are the "branching points" of the grammar.

use crate::{
    functions::{FunctionCall, FunctionDeclaration, FunctionExpression},
    terminals::{Identifier, IntegerLiteral},
};
use crate::Spanned;
use serde::Serialize;

/// The expression is the basic unit of computation.
///
/// # Grammar
///
/// ```pest,no_run
/// Expression =
///     { FunctionExpression
///     | FunctionCall
///     | Identifier
///     | IntegerLiteral
///     | __incomplete
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned)]
#[allow(missing_docs)]
pub enum Expression<'a> {
    FunctionExpression(FunctionExpression<'a>),
    FunctionCall(FunctionCall<'a>),
    Identifier(Identifier<'a>),
    IntegerLiteral(IntegerLiteral<'a>),
}

/// A statement is the basic unit of procedural work.
///
/// # Grammar
///
/// ```pest,no_run
/// Statement =
///     { ( Expression
///       ~ ";"
///       )
///     | FunctionDeclaration
///     | __incomplete
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned)]
#[allow(missing_docs)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    FunctionDeclaration(FunctionDeclaration<'a>),
}
