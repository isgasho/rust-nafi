use ast::{declarations::Declaration, expressions::Expression, Span};
use pest::iterators::Pair;
use single::Single;
use syntax::Rule;

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    Declaration(Declaration<'a>),
}

impl<'a> Statement<'a> {
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::Statement);
        let inner = parse.into_inner().single().unwrap();
        match inner.as_rule() {
            Rule::Expression => Statement::Expression(Expression::from_pest(inner)),
            Rule::Declaration => Statement::Declaration(Declaration::from_pest(inner)),
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

impl<'a> StatementBlock<'a> {
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::StatementBlock);
        let span = parse.as_span();
        let inner = parse.into_inner();
        let mut block = StatementBlock {
            span: Span::from_pest(span),
            statements: vec![],
            tail: None,
        };
        for parse in inner {
            match parse.as_rule() {
                Rule::Statement => block.statements.push(Statement::from_pest(parse)),
                Rule::Expression => block.tail = Some(Expression::from_pest(parse)),
                _ => unreachable!("Unexpected StatementBlock[{:?}]", parse.as_rule()),
            }
        }
        block
    }
}
