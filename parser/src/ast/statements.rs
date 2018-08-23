use crate::{
    ast::{
        declarations::Declaration, expressions::Expression, from_pest, FromPest, PestDeconstruct,
        Span,
    },
    syntax::Rule,
};
use {pest::iterators::Pair, serde_derive::Serialize, single::Single};

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    Declaration(Declaration<'a>),
}

impl<'a> FromPest<'a> for Statement<'a> {
    const RULE: Rule = Rule::Statement;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let inner = parse.into_inner().single().unwrap();
        match inner.as_rule() {
            Rule::Expression => Statement::Expression(from_pest(inner)),
            Rule::Declaration => Statement::Declaration(from_pest(inner)),
            rule => unimplemented!("Unexpected Statement[{:?}]", rule),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct StatementBlock<'a> {
    pub span: Span<'a>,
    pub statements: Vec<Statement<'a>>,
    pub tail: Option<Expression<'a>>,
}

impl<'a> FromPest<'a> for StatementBlock<'a> {
    const RULE: Rule = Rule::StatementBlock;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        let mut inner = parse.deconstruct();
        StatementBlock {
            span: Span::from_pest(span),
            statements: inner.next_many(),
            tail: inner.next_opt(),
        }
    }
}
