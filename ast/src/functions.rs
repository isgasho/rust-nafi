//! Functions are units of behavior. They take input, do some work, and produce output.

use crate::terminals::Identifier;
use crate::paths::Path;
use crate::{Span, Spanned};
use serde::Serialize;

/// A function expression is a brace-delimited block of code representing a callable function.
///
/// # Grammar
///
/// ```pest,no_run
/// FunctionExpression =
///     { "{"
///     ~ ( CommaSeparated(FunctionExpressionArgument)?
///       ~ "->"
///       )?
///     ~ Statement*
///     ~ Expression?
///     ~ "}"
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned)]
pub struct FunctionExpression<'a> {
    span: Span<'a>,
    arguments: Vec<FunctionExpressionArgument<'a>>,
    statements: Vec</*Statement<'a>*/ ()>,
    tail_expression: Option<Box</*Expression<'a>*/ ()>>,
}

/// Function Expression arguments are the bindings for inputs to the function.
///
/// # Grammar
///
/// ```pest,no_run
/// FunctionExpressionArgument =
///     { Identifier
///     ~ TypeAscription?
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned)]
pub struct FunctionExpressionArgument<'a> {
    span: Span<'a>,
    name: Identifier<'a>,
    r#type: Option<Box<Path<'a>>>,
}

/// A Function Call is an invocation of a function, passing arguments to produce work and output.
///
/// # Grammar
///
/// ```pest,no_run
/// FunctionCall =
///     { Identifier
///     ~ "("
///     ~ CommaSeparated(FunctionCallArgument)?
///     ~ ")"
///     ~ FunctionExpression?
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned)]
pub struct FunctionCall<'a> {
    span: Span<'a>,
    path: Identifier<'a>,
    arguments: Vec<FunctionCallArgument<'a>>,
    tail_closure: Option<Box<FunctionExpression<'a>>>
}

/// Function Call Arguments bind a function argument to some value expression.
///
/// # Grammar
///
/// ```pest,no_run
/// FunctionCallArgument =
///     { ( Identifier
///       ~ "="
///       )?
///     ~ Expression
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned)]
pub struct FunctionCallArgument<'a> {
    span: Span<'a>,
    name: Option<Identifier<'a>>,
    value: Box</*Expression<'a>*/ ()>
}

/// A Function Declaration creates a new argument-overloadable function binding.
///
/// # Grammar
///
/// ```pest,no_run
/// FunctionDeclaration =
///     { Keyword("function")
///     ~ Identifier
///     ~ "("
///     ~ CommaSeparated(FunctionDeclarationArgument)?
///     ~ ")"
///     ~ TypeAscription?
///     ~ "="
///     ~ FunctionExpression
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned)]
pub struct FunctionDeclaration<'a> {
    span: Span<'a>,
    name: Identifier<'a>,
    arguments: Vec<FunctionDeclarationArgument<'a>>,
    r#return: Option<Box<Path<'a>>>,
    body: Box<FunctionExpression<'a>>,
}

/// Function Declaration Arguments are the types and optional labels used to call the function.
///
/// # Grammar
///
/// ```pest,no_run
/// FunctionDeclarationArgument =
///     { ( Identifier
///       ~ TypeAscription
///       )
///     | Path
///     }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Spanned)]
pub struct FunctionDeclarationArgument<'a> {
    span: Span<'a>,
    name: Option<Identifier<'a>>,
    r#type: Box<Path<'a>>,
}
