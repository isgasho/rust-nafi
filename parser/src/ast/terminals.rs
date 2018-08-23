use crate::{
    ast::{FromPest, Span},
    syntax::Rule,
};
use {pest::iterators::Pair, serde_derive::Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct Identifier<'a> {
    span: Span<'a>,
}

impl<'a> FromPest<'a> for Identifier<'a> {
    const RULE: Rule = Rule::Identifier;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        Identifier {
            span: Span::from_pest(span),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct IntegerLiteral<'a> {
    span: Span<'a>,
}

impl<'a> FromPest<'a> for IntegerLiteral<'a> {
    const RULE: Rule = Rule::IntegerLiteral;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        let span = parse.as_span();
        IntegerLiteral {
            span: Span::from_pest(span),
        }
    }
}
