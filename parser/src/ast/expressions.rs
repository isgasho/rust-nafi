use crate::{
    ast::{
        from_pest,
        statements::StatementBlock,
        terminals::{Identifier, IntegerLiteral},
        types::Type,
        FromPest, Span, PestDeconstruct
    },
    syntax::Rule,
};
use {pest::iterators::Pair, serde_derive::Serialize, single::Single};

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    IntegerLiteral(IntegerLiteral<'a>),
    Function(Function<'a>),
    FunctionCall(FunctionCall<'a>),
}

impl<'a> FromPest<'a> for Expression<'a> {
    const RULE: Rule = Rule::Expression;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let inner = parse.into_inner().single().unwrap();
        match inner.as_rule() {
            Rule::Identifier => Expression::Identifier(from_pest(inner)),
            Rule::IntegerLiteral => Expression::IntegerLiteral(from_pest(inner)),
            Rule::FunctionExpression => Expression::Function(from_pest(inner)),
            Rule::FunctionCall => Expression::FunctionCall(from_pest(inner)),
            rule => unreachable!("Unexpected Expression[{:?}]", rule),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct Function<'a> {
    pub span: Span<'a>,
    pub arguments: Vec<FunctionArgument<'a>>,
    pub body: Box<StatementBlock<'a>>,
}

impl<'a> FromPest<'a> for Function<'a> {
    const RULE: Rule = Rule::FunctionExpression;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        let mut inner = parse.deconstruct();
        Function {
            span: Span::from_pest(span),
            arguments: inner.next_or_default(),
            body: Box::new(inner.next()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct FunctionArgument<'a> {
    pub span: Span<'a>,
    pub name: Identifier<'a>,
    #[serde(rename = "type")]
    pub type_: Option<Type<'a>>,
}

impl<'a> FromPest<'a> for Vec<FunctionArgument<'a>> {
    const RULE: Rule = Rule::FunctionExpressionArguments;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        parse.deconstruct().next_many()
    }
}

impl<'a> FromPest<'a> for FunctionArgument<'a> {
    const RULE: Rule = Rule::FunctionExpressionArgument;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        let mut inner = parse.deconstruct();
        FunctionArgument {
            span: Span::from_pest(span),
            name: inner.next(),
            type_: inner.next_opt(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct FunctionCall<'a> {
    pub span: Span<'a>,
    pub function: Identifier<'a>,
    pub arguments: Vec<FunctionCallArgument<'a>>,
    pub closure: Option<Function<'a>>,
}

impl<'a> FromPest<'a> for Vec<FunctionCallArgument<'a>> {
    const RULE: Rule = Rule::FunctionCallArguments;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        parse.deconstruct().next_many()
    }
}

impl<'a> FromPest<'a> for FunctionCall<'a> {
    const RULE: Rule = Rule::FunctionCall;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        let mut inner = parse.deconstruct();
        FunctionCall {
            span: Span::from_pest(span),
            function: inner.next(),
            arguments: inner.next_or_default(),
            closure: inner.next_opt(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct FunctionCallArgument<'a> {
    pub span: Span<'a>,
    pub label: Option<Identifier<'a>>,
    pub value: Expression<'a>,
}

impl<'a> FromPest<'a> for FunctionCallArgument<'a> {
    const RULE: Rule = Rule::FunctionCallArgument;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        let mut inner = parse.deconstruct();
        FunctionCallArgument {
            span: Span::from_pest(span),
            label: inner.next_opt(),
            value: inner.next(),
        }
    }
}
