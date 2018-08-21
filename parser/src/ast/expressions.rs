use crate::{
    ast::{
        statements::StatementBlock,
        terminals::{Identifier, IntegerLiteral},
        types::Type,
        Span,
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
    Empty(Span<'a>),
}

impl<'a> Expression<'a> {
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::Expression);
        let outer_span = parse.as_span();
        match parse.into_inner().single() {
            Err(::single::Error::NoElements) => Expression::Empty(Span::from_pest(outer_span)),
            Err(::single::Error::MultipleElements) => unreachable!(),
            Ok(parse) => match parse.as_rule() {
                Rule::Identifier => Expression::Identifier(Identifier::from_pest(parse)),
                Rule::IntegerLiteral => {
                    Expression::IntegerLiteral(IntegerLiteral::from_pest(parse))
                }
                Rule::FunctionExpression => Expression::Function(Function::from_pest(parse)),
                Rule::FunctionCall => Expression::FunctionCall(FunctionCall::from_pest(parse)),
                rule => unreachable!("Unexpected Expression[{:?}]", rule),
            },
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

impl<'a> Function<'a> {
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::FunctionExpression);
        let span = parse.as_span();
        let mut inner = parse.into_inner();
        let (arguments, body) = {
            let mut body = inner.next().unwrap();
            let arguments = if body.as_rule() == Rule::FunctionExpressionArguments {
                let temp = body;
                body = inner.next().unwrap();
                Some(temp)
            } else {
                None
            };
            assert_eq!(body.as_rule(), Rule::StatementBlock);
            (arguments, body)
        };
        Function {
            span: Span::from_pest(span),
            arguments: arguments
                .into_iter()
                .flat_map(Pair::into_inner)
                .map(FunctionArgument::from_pest)
                .collect(),
            body: Box::new(StatementBlock::from_pest(body)),
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

impl<'a> FunctionArgument<'a> {
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::FunctionExpressionArgument);
        let span = parse.as_span();
        let mut inner = parse.into_inner();
        let name = Identifier::from_pest(inner.next().unwrap());
        let type_ = inner.next().map(Type::from_pest);
        FunctionArgument {
            span: Span::from_pest(span),
            name,
            type_,
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

impl<'a> FunctionCall<'a> {
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::FunctionCall);
        let span = parse.as_span();
        let mut inner = parse.into_inner().fuse();
        let function = inner.next().unwrap();
        let (arguments, closure) = match (inner.next(), inner.next()) {
            (Some(it), None) => {
                if it.as_rule() == Rule::FunctionExpression {
                    (None, Some(it))
                } else {
                    assert_eq!(it.as_rule(), Rule::FunctionCallArguments);
                    (Some(it), None)
                }
            }
            (arguments, closure) => (arguments, closure),
        };
        FunctionCall {
            span: Span::from_pest(span),
            function: Identifier::from_pest(function),
            arguments: arguments
                .into_iter()
                .flat_map(Pair::into_inner)
                .map(FunctionCallArgument::from_pest)
                .collect(),
            closure: closure.map(Function::from_pest),
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

impl<'a> FunctionCallArgument<'a> {
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::FunctionCallArgument);
        let span = parse.as_span();
        let mut inner = parse.into_inner();
        let (label, value) = {
            let mut value = inner.next().unwrap();
            let label = if value.as_rule() == Rule::Identifier {
                let temp = value;
                value = inner.next().unwrap();
                Some(temp)
            } else {
                None
            };
            assert_eq!(value.as_rule(), Rule::Expression);
            (label, value)
        };
        FunctionCallArgument {
            span: Span::from_pest(span),
            label: label.map(Identifier::from_pest),
            value: Expression::from_pest(value),
        }
    }
}
