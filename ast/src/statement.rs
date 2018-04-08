use location::Span;

use {Declaration, Expression, Identifier};

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Statement<'a> {
    Declaration(#[serde(borrow)] Declaration<'a>),
    Assignment(#[serde(borrow)] Assignment<'a>),
    Expression(#[serde(borrow)] Expression<'a>),
}

impl<'a> Statement<'a> {
    pub fn span(&self) -> Span {
        match self {
            Statement::Declaration(decl) => decl.span(),
            Statement::Assignment(assign) => assign.span,
            Statement::Expression(expr) => expr.span(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Assignment<'a> {
    #[serde(borrow)]
    pub binding: Identifier<'a>,
    #[serde(borrow)]
    pub value: Expression<'a>,
    pub span: Span,
}
