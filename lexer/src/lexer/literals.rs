use lexer::unicode::decimal_number;
use tokens::Token;
use nom::IResult;

/// Token::IntegerLiteral
named! {
    pub integer_literal<&str, Token>,
    do_parse!(
        num: decimal_number >>
        (Token::IntegerLiteral(num))
    )
}

// NOTE: Allow other quotation marks <https://unicode-table.com/en/sets/quotation-marks/> ?
// NOTE: Other quotation marks might be used as special string-like literals
#[allow(unused)]
/// Token::StringLiteral
fn string_literal(input: &str) -> IResult<&str, Token> {
    match tag!(input, "\"") {
        IResult::Done(mut i, _) => {
            let mut string = String::with_capacity(i.find('"').unwrap_or(i.len()));
            loop {
                match take!(i, 1) {
                    IResult::Done(_i, o) if o == "\\" => {
                        // Escape
                    },
                    IResult::Done(_i, o) if o == "\"" => {
                        // Quote end
                    },
                    IResult::Done(_i, o) => {
                        // Regular string contents
                        string.push_str(o);
                        i = _i;
                    },
                    IResult::Incomplete(_) => {
                        debug_assert_eq!(i, "");
                        // Allow eof to close string
                        return IResult::Done(i, Token::StringLiteral(string));
                    },
                    e @ IResult::Error(_) => return e.map(|_| unreachable!()),
                }
            }
        },
        result => result.map(|_| unreachable!())
    }
}
