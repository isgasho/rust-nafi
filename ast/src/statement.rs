use location::Span;

use {Expression, Identifier};

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Statement<'a> {
    Declaration(#[serde(borrow)] Declaration<'a>),
    Assignment(#[serde(borrow)] Assignment<'a>),
    Expression(#[serde(borrow)] Expression<'a>),
}

impl<'a> Statement<'a> {
    /// The start position of this expression.
    pub fn span(&self) -> Span {
        match *self {
            Statement::Declaration(ref decl) => decl.span,
            Statement::Assignment(ref assign) => assign.span,
            Statement::Expression(ref expr) => expr.span(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Deref, DerefMut)]
pub struct Declaration<'a>(#[serde(borrow)] pub Assignment<'a>);

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Assignment<'a> {
    #[serde(borrow)]
    pub binding: Identifier<'a>,
    #[serde(borrow)]
    pub value: Expression<'a>,
    pub span: Span,
}
