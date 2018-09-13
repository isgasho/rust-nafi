#![feature(tool_lints)]

pub mod ast;
mod syntax;

pub fn parse(s: &str) -> Result<ast::statements::StatementBlock, Box<dyn std::error::Error>> {
    use crate::{
        ast::from_pest,
        syntax::{NafiParser, Rule},
    };
    use pest::Parser;

    let parse = NafiParser::parse(Rule::TestEntry, s)?.next().unwrap();
    Ok(from_pest(parse))
}
