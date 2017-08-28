use std::{char, u32};

use error::*;
use lexer::unicode::decimal_number;
use nnom::prelude::{ParseOutput, PositionedStr, Result};
use tokens::{StringFragments, Token};

/// Token::IntegerLiteral
pub fn integer_literal(input: PositionedStr) -> Result<PositionedStr, Token, Error> {
    decimal_number(input)
        .map(
            |ParseOutput {
                 output: (pos, integer),
                 remaining_input,
             }| {
                ParseOutput {
                    output: Token::IntegerLiteral(pos, integer),
                    remaining_input,
                }
            },
        )
        .chain_err(|| {
            ErrorKind::NoMatch(input.start(), "lexer::literals::integer_literal")
        })
}

// NOTE: Allow other quotation marks <https://unicode-table.com/en/sets/quotation-marks/> ?
// NOTE: Other quotation marks might be used as special string-like literals
/// Token::StringLiteral
pub fn string_literal(input: PositionedStr) -> Result<PositionedStr, Token, Error> {
    if !input.starts_with("\"") {
        bail!(ErrorKind::NoMatch(
            input.start(),
            "lexer::literals::string_literal"
        ));
    }

    let pos = input.start();
    let mut remaining_input = input.split_at(1).1;
    let mut string = StringFragments::new();

    loop {
        match remaining_input.chars().next() {
            Some('\"') => {
                remaining_input = remaining_input.split_at(1).1;
                break;
            },
            Some('\\') => {
                let simple_escape_len = remaining_input.chars().take(2).map(char::len_utf8).sum();
                let (escape, mut rest) = remaining_input.split_at(simple_escape_len);
                match &*escape {
                    "\\\\" => string.push('\\'),
                    "\\\"" => string.push('\"'),
                    "\\n" => string.push('\n'),
                    "\\t" => string.push('\t'),
                    "\\u" => {
                        // FIXME: refactor: extract fn
                        if rest.starts_with("{") {
                            if let Some(close_idx) = rest.find("}") {
                                let split = rest.split_at(close_idx + 1);
                                rest = split.1;
                                let codepoint = &split.0[1..close_idx];
                                if let Ok(Some(char)) =
                                    u32::from_str_radix(codepoint, 16).map(char::from_u32)
                                {
                                    string.push(char)
                                } else {
                                    string.push_invalid_escape(&remaining_input[1..close_idx + 3])
                                }
                            } else {
                                let split = remaining_input.split_at(remaining_input.len());
                                rest = split.1;
                                string.push_invalid_escape(&split.0[1..]);
                            }
                        } else {
                            string.push_invalid_escape("u")
                        }
                    },
                    s @ _ => string.push_invalid_escape(&s[1..]),
                }
                remaining_input = rest;
            },
            Some(_) => {
                let first_interesting_index = remaining_input
                    .find(|c| c == '\"' || c == '\\')
                    .unwrap_or(remaining_input.len());
                let (uninteresting, rest) = remaining_input.split_at(first_interesting_index);
                string.push_str(&*uninteresting);
                remaining_input = rest;
            },
            None => break,
        }
    }

    Ok(ParseOutput {
        remaining_input,
        output: Token::StringLiteral(pos, string),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_escapes() {
        assert_eq!(
            string_literal("\"\\\"\"".into()).map_err(|e| e.to_string()), // "\""
            Ok(ParseOutput {
                output: Token::StringLiteral(0, "\"".into()),
                remaining_input: PositionedStr::new("", 4),
            })
        );
        assert_eq!(
            string_literal("\"\\n\"".into()).map_err(|e| e.to_string()), // "\n"
            Ok(ParseOutput {
                output: Token::StringLiteral(0, "\n".into()),
                remaining_input: PositionedStr::new("", 4),
            })
        );
        assert_eq!(
            string_literal("\"\\t\"".into()).map_err(|e| e.to_string()), // "\t"
            Ok(ParseOutput {
                output: Token::StringLiteral(0, "\t".into()),
                remaining_input: PositionedStr::new("", 4),
            })
        );
        assert_eq!(
            string_literal("\"\\u{FFFF}\"".into()).map_err(|e| e.to_string()), // "\u{FFFF}"
            Ok(ParseOutput {
                output: Token::StringLiteral(0, "\u{FFFF}".into()),
                remaining_input: PositionedStr::new("", 10),
            })
        );
    }

    #[test]
    fn invalid_string_escapes() {
        assert_eq!(
            string_literal("\"\\u{}\"".into()).map_err(|e| e.to_string()), // "\u{}"
            Ok(ParseOutput {
                remaining_input: PositionedStr::new("", 6),
                output: Token::StringLiteral(0, {
                    let mut fragments = StringFragments::new();
                    fragments.push_invalid_escape("u{}");
                    fragments
                }),
            })
        );
        assert_eq!(
            string_literal("\"\\u{110000}\"".into()).map_err(|e| e.to_string()), // "\u{110000}"
            Ok(ParseOutput {
                remaining_input: PositionedStr::new("", 12),
                output: Token::StringLiteral(0, {
                    let mut fragments = StringFragments::new();
                    fragments.push_invalid_escape("u{110000}");
                    fragments
                }),
            })
        );
        assert_eq!(
            string_literal("\"\\u{XXXX}\"".into()).map_err(|e| e.to_string()), // "\u{XXXX}"
            Ok(ParseOutput {
                remaining_input: PositionedStr::new("", 10),
                output: Token::StringLiteral(0, {
                    let mut fragments = StringFragments::new();
                    fragments.push_invalid_escape("u{XXXX}");
                    fragments
                }),
            })
        );
    }
}
