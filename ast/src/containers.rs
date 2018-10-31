//! Containers (for lack of a better module for them) are the "branching points" of the grammar.

use crate::{
    functions::{FunctionCall, FunctionDeclaration, FunctionExpression},
    parser::Rule,
    terminals::{Identifier, IntegerLiteral},
    Spanned,
};
use pest_ast::FromPest;
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
#[derive(Serialize, Spanned, FromPest)]
#[pest_ast(rule(Rule::Expression))]
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
#[derive(Serialize, Spanned, FromPest)]
#[pest_ast(rule(Rule::Statement))]
#[allow(missing_docs)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    FunctionDeclaration(FunctionDeclaration<'a>),
}
