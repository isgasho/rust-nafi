use lexer::unicode::decimal_number;
use tokens::Token;

/// Token::IntegerLiteral
named! {
    pub integer_literal<&str, Token>,
    do_parse!(
        num: decimal_number >>
        (Token::IntegerLiteral(num))
    )
}
