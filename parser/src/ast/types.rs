use ast::terminals::Identifier;
use pest::iterators::Pair;
use single::Single;
use syntax::Rule;

#[derive(Clone, Debug, Eq, PartialEq)]
#[derive(Serialize)]
pub enum Type<'a> {
    Identifier(Identifier<'a>),
}

impl<'a> Type<'a> {
    pub(crate) fn from_pest(parse: Pair<'a, Rule>) -> Self {
        assert_eq!(parse.as_rule(), Rule::Type);
        let inner = parse.into_inner().single().unwrap();
        assert_eq!(inner.as_rule(), Rule::Identifier);
        Type::Identifier(Identifier::from_pest(inner))
    }
}
