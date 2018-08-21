pub mod declarations;
pub mod expressions;
pub mod statements;
pub mod terminals;
pub mod types;

mod span;

pub use self::span::Span;

use crate::syntax::Rule;
use pest::iterators::Pair;

pub(crate) fn from_pest<'a, T: FromPest<'a>>(parse: Pair<'a, Rule>) -> T {
    assert_eq!(parse.as_rule(), T::RULE);
    T::from_pest(parse)
}

pub(crate) trait FromPest<'a> {
    const RULE: Rule;
    fn from_pest(parse: Pair<'a, Rule>) -> Self;
}
