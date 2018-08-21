use crate::{
    ast::{
        expressions::{Expression, Function as FunctionExpression},
        from_pest,
        terminals::Identifier,
        types::Type,
        FromPest, Span,
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
        assert_eq!(parse.as_rule(), Rule::Declaration);
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
        assert_eq!(parse.as_rule(), Rule::FunctionDeclaration);
        let span = parse.as_span();
        let mut inner = parse.into_inner();
        let name = inner.next().unwrap();
        let arguments = inner.next().unwrap();
        let (return_, body) = {
            let mut body = inner.next().unwrap();
            let return_ = if body.as_rule() == Rule::TypeAscription {
                let temp = body;
                body = inner.next().unwrap();
                Some(temp)
            } else {
                None
            };
            assert_eq!(body.as_rule(), Rule::FunctionExpression);
            (return_, body)
        };
        Function {
            span: Span::from_pest(span),
            name: from_pest(name),
            arguments: arguments.into_inner().map(from_pest).collect(),
            return_: return_
                .map(Pair::into_inner)
                .map(Single::single)
                .map(Result::unwrap)
                .map(from_pest),
            body: from_pest(body),
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

impl<'a> FromPest<'a> for FunctionArgument<'a> {
    const RULE: Rule = Rule::FunctionDeclarationArgument;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::FunctionDeclarationArgument);
        let span = parse.as_span();
        let mut inner = parse.into_inner();
        let (name, type_) = {
            let mut type_ = inner.next().unwrap();
            let name = if type_.as_rule() == Rule::Identifier {
                let temp = type_;
                type_ = inner.next().unwrap();
                Some(temp)
            } else {
                None
            };
            (name, type_)
        };
        FunctionArgument {
            span: Span::from_pest(span),
            name: name.map(from_pest),
            type_: from_pest(type_),
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
        assert_eq!(parse.as_rule(), Rule::LetDeclaration);
        let span = parse.as_span();
        let mut inner = parse.into_inner();
        let name = inner.next().unwrap();
        let (type_, value) = {
            let mut value = inner.next().unwrap();
            let type_ = if value.as_rule() == Rule::TypeAscription {
                let temp = value;
                value = inner.next().unwrap();
                Some(temp)
            } else {
                None
            };
            (type_, value)
        };
        Let {
            span: Span::from_pest(span),
            name: from_pest(name),
            type_: type_
                .map(Pair::into_inner)
                .map(Single::single)
                .map(Result::unwrap)
                .map(from_pest),
            value: from_pest(value),
        }
    }
}
