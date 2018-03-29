use bigint;
use tokens::{Position, StringFragments, Token};

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Expression<'a> {
    Identifier(Token<'a>),
    Parenthesized(Box<ParenthesizedExpression<'a>>),
    Operator(Box<OperatorExpression<'a>>),
    Literal(Box<LiteralExpression<'a>>),
}

impl<'a> Expression<'a> {
    /// The start position of this expression.
    pub fn position(&self) -> Position {
        match *self {
            Expression::Identifier(ref token) => token.position,
            Expression::Parenthesized(ref expr) => expr.position(),
            Expression::Operator(ref expr) => expr.position(),
            Expression::Literal(ref expr) => expr.position(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct ParenthesizedExpression<'a> {
    left_paren: Token<'a>,
    inner: Expression<'a>,
    right_paren: Token<'a>,
}

impl<'a> ParenthesizedExpression<'a> {
    /// The start position of this expression.
    pub fn position(&self) -> Position { self.left_paren.position }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct OperatorExpression<'a> {
    lhs: Expression<'a>,
    operator: Vec<Token<'a>>,
    rhs: Expression<'a>,
}

impl<'a> OperatorExpression<'a> {
    /// The start position of this expression.
    pub fn position(&self) -> Position { self.lhs.position() }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum LiteralExpression<'a> {
    Integer(Token<'a>, bigint::BigInt),
    String(Token<'a>, StringFragments<'a>),
}

impl<'a> LiteralExpression<'a> {
    /// The start position of this expression.
    pub fn position(&self) -> Position {
        match *self {
            LiteralExpression::Integer(ref token, _) | LiteralExpression::String(ref token, _) => {
                token.position
            },
        }
    }
}
