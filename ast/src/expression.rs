use bigint;
use tokens;

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Expression {
    Identifier(usize, tokens::Identifier),
    Parenthesized(usize, Box<Expression>),
    Operator(usize, Box<OperatorExpression>),
    Literal(usize, Box<LiteralExpression>),
}

impl Expression {
    /// The start position of this expression.
    pub fn position(&self) -> usize {
        match *self {
            Expression::Identifier(pos, _)
            | Expression::Parenthesized(pos, _)
            | Expression::Operator(pos, _)
            | Expression::Literal(pos, _) => pos,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum LiteralExpression {
    Integer(bigint::BigInt),
    String(tokens::StringFragments),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct OperatorExpression {
    lhs: Expression,
    rhs: Expression,
    op: Operator,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct Operator(Vec<tokens::Symbol>);
