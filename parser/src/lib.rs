pub mod ast;
mod syntax;

pub fn parse(s: &str) -> Result<ast::expressions::Expression<'_>, Box<dyn std::error::Error>> {
    use crate::syntax::{NafiParser, Rule};
    use pest::Parser;
    let parse = {
        let mut pairs = NafiParser::parse(Rule::TestEntry, s)?;
        let pair = pairs.next().unwrap();
        assert_eq!(pairs.next().unwrap().as_rule(), Rule::EOI);
        pair
    };
    Ok(ast::expressions::Expression::from_pest(parse))
}
