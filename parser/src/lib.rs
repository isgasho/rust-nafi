pub use nafi_ast as ast;

pub fn parse(s: &str) -> Result<ast::functions::FunctionExpression, Box<dyn std::error::Error>> {
    use pest_deconstruct::FromPest;
    use crate::ast::parser::{Parser, Rule};
    use pest::Parser as Parse;

    let mut parse = Parser::parse(Rule::FunctionExpression, s)?;
    assert!(s[parse.as_str().len()..].chars().all(|c| c.is_whitespace()));
    Ok(FromPest::from_pest(parse.next().unwrap()))
}
