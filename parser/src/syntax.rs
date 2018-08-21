use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"]
pub(crate) struct NafiParser;
const _GRAMMAR: &str = include_str!("syntax.pest");
