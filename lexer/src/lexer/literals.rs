use super::unicode::decimal_number;
use lexer::tokens;
use nnom::{ParseOutput, ParseResult};
use nnom::slice::PositionedStr;
//use std::u32;
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

// FIXME BUG: String interpolation containing a string with unbalanced {} breaks everything help
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
                    Some('{') => {
                        let block = grab_matched_substring('{', '}')(remaining_input)
                            .unwrap()
                            .output;
                        let Ok(ParseOutput { mut output, .. }) = tokens(block);
                        // Remove leading bracket
                        output.remove(0);
                        // Remove trailing bracket (can be absent if closed by EOF)
                        if let Some(&Token::Symbol(_, Symbol::RightCurlyBracket)) = output.last() {
                            output.pop();
                        }
                        string.push_interpolation(output);
                        remaining_input = remaining_input.split_at(block.len()).1;
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

fn grab_matched_substring(
    open: char,
    close: char,
) -> impl Fn(PositionedStr) -> ParseResult<PositionedStr, PositionedStr, ()> {
    move |input: PositionedStr| {
        if !input.starts_with(open) {
            return Err(());
        }

        let mut remaining_input = input.split_at(open.len_utf8()).1;
        let mut depth = 1u32;

        loop {
            match remaining_input.chars().next() {
                Some(ch) if ch == close => {
                    remaining_input = remaining_input.split_at(ch.len_utf8()).1;
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                },
                Some(ch) if ch == open => {
                    remaining_input = remaining_input.split_at(ch.len_utf8()).1;
                    depth += 1;
                },
                Some(_) => {
                    let idx = remaining_input
                        .find(|ch| ch == open || ch == close)
                        .unwrap_or_else(|| remaining_input.len());
                    remaining_input = remaining_input.split_at(idx).1;
                },
                None => break,
            }
        }

        Ok(ParseOutput {
            remaining_input,
            output: input.split_at(remaining_input.start() - input.start()).0,
        })
    }
}
