use crate::{
    ast::{from_pest, terminals::Identifier, FromPest},
    syntax::Rule,
};
use {pest::iterators::Pair, serde_derive::Serialize, single::Single};

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub enum Type<'a> {
    Identifier(Identifier<'a>),
}

impl<'a> FromPest<'a> for Type<'a> {
    const RULE: Rule = Rule::Type;
    fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::Type);
        let inner = parse.into_inner().single().unwrap();
        assert_eq!(inner.as_rule(), Rule::Identifier);
        Type::Identifier(from_pest(inner))
    }
}
