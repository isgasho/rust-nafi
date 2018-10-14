use crate::{containers::Expression, functions::FunctionExpression, parser::Rule, paths::Path};
use pest::iterators::Pair;
use pest_deconstruct::FromPest;

macro_rules! shim {
    ($($T:ident)*) => {$(
        impl<'a> FromPest<'a> for Box<$T<'a>> {
            type Rule = Rule;
            const RULE: Rule = $T::RULE;
            fn from_pest(pest: Pair<'a, Rule>) -> Self {
                Box::new($T::from_pest(pest))
            }
        }
    )*};
}

shim! {
    Expression
    FunctionExpression
    Path
}
