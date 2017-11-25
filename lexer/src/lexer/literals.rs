use super::unicode::decimal_number;
use lexer::token;
use nnom::{ParseOutput, ParseResult};
use nnom::slice::PositionedStr;
use std::{char, u32};
use tokens::{StringFragments, Symbol, Token};

/// `Token::Literal(Literal::Integer)`
pub fn integer_literal(input: PositionedStr) -> ParseResult<PositionedStr, Token, ()> {
    decimal_number(input).map(
        |ParseOutput {
             remaining_input,
             output,
         }| {
            ParseOutput {
                remaining_input,
                output: Token::Literal(input.start(), output.into()),
            }
        },
    )
}

// TODO: Break single-character escapes into reusable fn and build character_literal
/// `Token::Literal(Literal::String)`
pub fn string_literal(input: PositionedStr) -> ParseResult<PositionedStr, Token, ()> {
    if !input.starts_with('"') {
        return Err(());
    }

    let mut remaining_input = input.split_at(1).1;
    let mut string = StringFragments::new();

    loop {
        match remaining_input.chars().next() {
            Some('\"') => {
                remaining_input = remaining_input.split_at(1).1;
                break;
            },
            Some('\\') => {
                remaining_input = remaining_input.split_at(1).1;
                match remaining_input.chars().next() {
                    Some(ch) if matches!(ch, '\\' | '\"' | '\'') => {
                        string.push_char(ch);
                        remaining_input = remaining_input.split_at(1).1;
                    },
                    Some('r') => {
                        string.push_char('\r');
                        remaining_input = remaining_input.split_at(1).1;
                    },
                    Some('n') => {
                        string.push_char('\n');
                        remaining_input = remaining_input.split_at(1).1;
                    },
                    Some('t') => {
                        string.push_char('\t');
                        remaining_input = remaining_input.split_at(1).1;
                    },
                    Some('u') => {
                        if remaining_input.starts_with("u{") {
                            if let Some(idx) = remaining_input.find('}') {
                                let codepoint = u32::from_str_radix(&remaining_input[2..idx], 16);
                                if let Ok(Some(ch)) = codepoint.map(char::from_u32) {
                                    string.push_char(ch);
                                } else {
                                    string.push_invalid_escape(&remaining_input[..idx + 1])
                                }
                                remaining_input = remaining_input.split_at(idx).1;
                            } else {
                                string.push_invalid_escape('u'.to_string());
                            }
                        } else {
                            string.push_invalid_escape('u'.to_string());
                        }
                        remaining_input = remaining_input.split_at(1).1;
                    },
                    Some('{') => {
                        remaining_input = remaining_input.split_at(1).1;
                        let mut tokens: Vec<Token> = vec![];
                        let mut depth = 1u32;
                        while let Ok(ParseOutput {
                            remaining_input: tail,
                            output,
                        }) = token(remaining_input)
                        {
                            remaining_input = tail;
                            match output {
                                Token::Symbol(_, Symbol::LeftCurlyBracket) => depth += 1,
                                Token::Symbol(_, Symbol::RightCurlyBracket) => {
                                    depth -= 1;
                                    if depth == 0 {
                                        break;
                                    }
                                },
                                _ => {},
                            }
                            tokens.push(output);
                        }
                        string.push_interpolation(tokens);
                    },
                    Some(s) => string.push_invalid_escape(s.to_string()),
                    None => break,
                }
            },
            Some(_) => {
                let idx = remaining_input
                    .find(|ch| matches!(ch, '\"' | '\\'))
                    .unwrap_or_else(|| remaining_input.len());
                let (uninteresting, rest) = remaining_input.split_at(idx);
                string.push_string(&*uninteresting);
                remaining_input = rest;
            },
            None => break,
        }
    }

    Ok(ParseOutput {
        remaining_input,
        output: Token::Literal(input.start(), string.into()),
    })
}
