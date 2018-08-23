use crate::{
    ast::{
        expressions::{Expression, Function as FunctionExpression},
        from_pest,
        terminals::Identifier,
        types::Type,
        FromPest, PestDeconstruct, Span,
    },
    syntax::Rule,
};
use {pest::iterators::Pair, serde_derive::Serialize, single::Single};

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub enum Declaration<'a> {
    Function(Function<'a>),
    Let(Let<'a>),
}

impl<'a> FromPest<'a> for Declaration<'a> {
    const RULE: Rule = Rule::Declaration;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let inner = parse.into_inner().single().unwrap();
        match inner.as_rule() {
            Rule::FunctionDeclaration => Declaration::Function(from_pest(inner)),
            Rule::LetDeclaration => Declaration::Let(from_pest(inner)),
            rule => unreachable!("Unexpected Declaration[{:?}]", rule),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct Function<'a> {
    pub span: Span<'a>,
    pub name: Identifier<'a>,
    pub arguments: Vec<FunctionArgument<'a>>,
    #[serde(rename = "return")]
    pub return_: Option<Type<'a>>,
    pub body: FunctionExpression<'a>,
}

impl<'a> FromPest<'a> for Function<'a> {
    const RULE: Rule = Rule::FunctionDeclaration;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        let mut inner = parse.deconstruct();
        Function {
            span: Span::from_pest(span),
            name: inner.next(),
            arguments: inner.next(),
            return_: inner.next_opt(),
            body: inner.next(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct FunctionArgument<'a> {
    pub span: Span<'a>,
    pub name: Option<Identifier<'a>>,
    #[serde(rename = "type")]
    pub type_: Type<'a>,
}

impl<'a> FromPest<'a> for Vec<FunctionArgument<'a>> {
    const RULE: Rule = Rule::FunctionDeclarationArguments;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        parse.deconstruct().next_many()
    }
}

impl<'a> FromPest<'a> for FunctionArgument<'a> {
    const RULE: Rule = Rule::FunctionDeclarationArgument;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        let mut inner = parse.deconstruct();
        FunctionArgument {
            span: Span::from_pest(span),
            name: inner.next_opt(),
            type_: inner.next(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct Let<'a> {
    pub span: Span<'a>,
    pub name: Identifier<'a>,
    #[serde(rename = "type")]
    pub type_: Option<Type<'a>>,
    pub value: Expression<'a>,
}

impl<'a> FromPest<'a> for Let<'a> {
    const RULE: Rule = Rule::LetDeclaration;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        let mut inner = parse.deconstruct();
        Let {
            span: Span::from_pest(span),
            name: inner.next(),
            type_: inner.next_opt(),
            value: inner.next(),
        }
    }
}
