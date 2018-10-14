pub use nafi_ast as ast;

pub fn parse(s: &str) -> Result<ast::functions::FunctionExpression, Box<dyn std::error::Error>> {
    use crate::ast::parser::{Parser, Rule};
    use pest::Parser as Parse;
    use pest_deconstruct::FromPest;

    let mut parse = Parser::parse(Rule::FunctionExpression, s)?;
    assert!(s[parse.as_str().len()..].chars().all(|c| c.is_whitespace()));
    Ok(FromPest::from_pest(parse.next().unwrap()))
}
