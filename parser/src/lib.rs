pub mod ast;
mod syntax;

pub fn parse(s: &str) -> Result<ast::statements::StatementBlock, Box<dyn std::error::Error>> {
    use crate::{syntax::{NafiParser, Rule}, ast::from_pest};
    use pest::Parser;

    let parse = NafiParser::parse(Rule::TestEntry, s)?.next().unwrap();
    Ok(from_pest(parse))
}
