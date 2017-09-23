//! Syntax Tree for Nafi. VERY ROUGH currently.

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Declaration {
        binding: Path,
        mutable: bool,
        value: Expression,
    },
    Assignment {
        binding: Path,
        value: Expression,
    },
}

#[derive(Debug)]
pub struct Path(Vec<String>);

#[derive(Debug)]
pub enum Expression {
    Binding(String),
    LiteralString(String),
    LiteralInteger(u64),
    PrefixOperator(String, Box<Expression>),
    PostfixOperator(Box<Expression>, String),
    BinaryOperator(Box<Expression>, String, Box<Expression>),
}
