use crate::{ast::Span, syntax::Rule};
use {pest::iterators::Pair, serde_derive::Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct Identifier<'a> {
    span: Span<'a>,
}

impl<'a> Identifier<'a> {
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::Identifier);
        Identifier {
            span: Span::from_pest(parse.as_span()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub struct IntegerLiteral<'a> {
    span: Span<'a>,
}

impl<'a> IntegerLiteral<'a> {
    #[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::IntegerLiteral);
        IntegerLiteral {
            span: Span::from_pest(parse.as_span()),
        }
    }
}
