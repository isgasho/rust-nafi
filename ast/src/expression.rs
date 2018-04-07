use bigint::BigInt;
use location::Span;

use {Identifier, Operator};

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Expression<'a> {
    Identifier(#[serde(borrow)] Box<Identifier<'a>>),
    Parenthesized(#[serde(borrow)] Box<Parenthesized<'a>>),
    BinaryOperator(#[serde(borrow)] Box<BinaryOperator<'a>>),
    IntegerLiteral(Box<IntegerLiteral>),
    StringLiteral(#[serde(borrow)] Box<StringLiteral<'a>>),
}

impl<'a> Expression<'a> {
    pub fn span(&self) -> Span {
        match *self {
            Expression::Identifier(ref expr) => expr.span,
            Expression::Parenthesized(ref expr) => expr.span,
            Expression::BinaryOperator(ref expr) => expr.span,
            Expression::IntegerLiteral(ref expr) => expr.span,
            Expression::StringLiteral(ref expr) => expr.span,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Parenthesized<'a> {
    #[serde(borrow)]
    pub inner: Expression<'a>,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct BinaryOperator<'a> {
    #[serde(borrow)]
    pub lhs: Expression<'a>,
    #[serde(borrow)]
    pub op: Operator<'a>,
    #[serde(borrow)]
    pub rhs: Expression<'a>,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct IntegerLiteral {
    pub value: BigInt,
    pub span: Span,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct StringLiteral<'a> {
    pub contents: &'a str,
    pub span: Span,
}
