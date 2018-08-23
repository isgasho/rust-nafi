pub mod declarations;
pub mod expressions;
pub mod statements;
pub mod terminals;
pub mod types;

mod span;

pub use self::span::Span;

use crate::syntax::Rule;
use pest::iterators::{Pair, Pairs};
use std::iter::Peekable;

pub(crate) fn from_pest<'a, T: FromPest<'a>>(parse: Pair<'a, Rule>) -> T {
    assert_eq!(parse.as_rule(), T::RULE);
    T::from_pest(parse)
}

pub(crate) trait FromPest<'a> {
    const RULE: Rule;
    fn from_pest(parse: Pair<'a, Rule>) -> Self;
}

pub(crate) trait PestDeconstruct<'i> {
    fn deconstruct(self) -> PestDeconstructor<'i>;
}

impl<'i> PestDeconstruct<'i> for Pair<'i, Rule> {
    fn deconstruct(self) -> PestDeconstructor<'i> {
        PestDeconstructor(self.into_inner().peekable())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PestDeconstructor<'i>(Peekable<Pairs<'i, Rule>>);

impl<'i> Drop for PestDeconstructor<'i> {
    fn drop(&mut self) {
        assert_eq!(
            self.0.next(),
            None,
            "PestDeconstructor was not fully exhausted"
        )
    }
}

impl<'i> PestDeconstructor<'i> {
    pub(crate) fn next<T: FromPest<'i>>(&mut self) -> T {
        self.next_opt().unwrap_or_else(|| {
            panic!(
                "expected {:?} child, got {:?}",
                T::RULE,
                self.0.next().map(|pair| pair.as_rule())
            )
        })
    }

    pub(crate) fn next_opt<T: FromPest<'i>>(&mut self) -> Option<T> {
        match self.0.peek() {
            Some(child) if child.as_rule() == T::RULE => Some(T::from_pest(self.0.next().unwrap())),
            _ => None,
        }
    }

    pub(crate) fn next_or_default<T: FromPest<'i> + Default>(&mut self) -> T {
        self.next_opt().unwrap_or_default()
    }

    pub(crate) fn next_many<T: FromPest<'i>>(&mut self) -> Vec<T> {
        let mut children = vec![];
        while let Some(child) = self.next_opt() {
            children.push(child);
        }
        children
    }

    #[allow(dead_code)]
    pub(crate) fn discard(self) {
        let _ = self.0;
    }
}
