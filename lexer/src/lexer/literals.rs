use std::{char, u32};

use lexer::unicode::decimal_number;
use tokens::{StringFragments, Token};

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
// FIXME: Monolith with i, _i, and __i
/// Token::StringLiteral
pub fn string_literal(input: &str) -> IResult<&str, Token> {
    match tag!(input, "\"") {
        IResult::Done(mut i, _) => {
            let mut string = StringFragments::new();

            loop {
                match take!(i, 1) {
                    IResult::Done(_i, o) if o == "\\" => {
                        match take!(_i, 1) {
                            IResult::Done(_i, o) if o == "\"" => {
                                string.push('\"');
                                i = _i;
                            },
                            IResult::Done(_i, o) if o == "\\" => {
                                string.push('\\');
                                i = _i;
                            },
                            IResult::Done(_i, o) if o == "n" => {
                                string.push('\n');
                                i = _i;
                            },
                            IResult::Done(_i, o) if o == "t" => {
                                string.push('\t');
                                i = _i;
                            },
                            IResult::Done(mut __i, o) if o == "u" => {
                                // FIXME: So many bad names and magic numbers
                                // FIXME: This should be a separate parser bit (string between {})
                                if __i.chars().next() == Some('{') {
                                    if let Some(idx) = __i.find('}') {
                                        let codepoint = &__i[1..idx];
                                        i = &__i[idx + 1..];
                                        if let Ok(Some(char)) =
                                            u32::from_str_radix(codepoint, 16).map(char::from_u32)
                                        {
                                            string.push(char);
                                        } else {
                                            string.push_invalid_escape(&_i[..idx + 2]);
                                        }
                                    } else {
                                        string.push_invalid_escape(_i);
                                        i = "";
                                    }
                                } else {
                                    string.push_invalid_escape("u");
                                    i = __i;
                                }
                            },
                            IResult::Done(_i, o) => {
                                string.push_invalid_escape(o);
                                i = _i;
                            },
                            _ => unimplemented!("Can this happen?"),
                        }
                    },
                    IResult::Done(_i, o) if o == "\"" => {
                        i = _i;
                        break;
                    },
                    IResult::Done(_i, o) => {
                        string.push_str(o);
                        i = _i;
                    },
                    IResult::Incomplete(_) => break, // Allow eof to close string
                    e @ IResult::Error(_) => return e.map(|_| unreachable!()),
                }
            }

            return IResult::Done(i, Token::StringLiteral(string));
        },
        result => result.map(|_| unreachable!()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_escapes() {
        assert_eq!(
            string_literal("\"\\\"\""), // "\""
            IResult::Done("", Token::StringLiteral("\"".into()))
        );
        assert_eq!(
            string_literal("\"\\n\""), // "\n"
            IResult::Done("", Token::StringLiteral("\n".into()))
        );
        assert_eq!(
            string_literal("\"\\t\""), // "\t"
            IResult::Done("", Token::StringLiteral("\t".into()))
        );
        assert_eq!(
            string_literal("\"\\u{FFFF}\""), // "\u{FFFF}"
            IResult::Done("", Token::StringLiteral("\u{FFFF}".into()))
        );
    }

    #[test]
    fn invalid_string_escapes() {
        assert_eq!(
            string_literal("\"\\u{}\""),
            IResult::Done(
                "",
                Token::StringLiteral({
                    let mut fragments = StringFragments::new();
                    fragments.push_invalid_escape("u{}");
                    fragments
                })
            )
        )
    }
}
