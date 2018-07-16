extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate bytecount;
extern crate memchr;
extern crate single;

pub mod ast;
mod syntax;

pub fn parse(s: &str) -> Result<ast::expressions::Expression, Box<::std::error::Error>> {
    use pest::Parser;
    use syntax::{NafiParser, Rule};
    let parse = {
        let mut pairs = NafiParser::parse(Rule::TestEntry, s)?;
        let pair = pairs.next().unwrap();
        assert_eq!(pairs.next().unwrap().as_rule(), Rule::EOI);
        pair
    };
    Ok(ast::expressions::Expression::from_pest(parse))
}
