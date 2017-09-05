//! Unicode adapters for nom parsers

use error::*;
use nnom::prelude::{ParseOutput, PositionedStr, Result};
use tokens::BigUint;

// FIXME: use an actual unicode rule
/// Recognize characters that are part of a newline
pub fn is_newline_char(char: char) -> bool { char == '\r' || char == '\n' }

// FIXME: match on Pattern_White_Space
/// Recognize whitespace characters
pub fn is_whitespace_char(char: char) -> bool {
    char == ' ' || char == '\t' || char == '\r' || char == '\n'
}

/// Recognize one or more unicode whitespaces
pub fn white_space(input: PositionedStr) -> Result<PositionedStr, PositionedStr, Error> {
    let mut index = None;

    for (idx, char) in input.char_indices() {
        if !is_whitespace_char(char) {
            index = Some(idx);
            break;
        }
    }

    let index = index.unwrap_or(input.len());

    if index == 0 {
        bail!(ErrorKind::NoMatch(
            input.start(),
            "lexer::unicode::white_space"
        ));
    }

    let split = input.split_at(index);
    Ok(ParseOutput {
        output: split.0,
        remaining_input: split.1,
    })
}

// FIXME: match on Numeric_Type=Decimal
/// Parse a decimal number
pub fn decimal_number(input: PositionedStr) -> Result<PositionedStr, (usize, BigUint), Error> {
    let len = input
        .char_indices()
        .find(|&(_, char)| !char.is_digit(10))
        .map(|(idx, _)| idx)
        .unwrap_or(input.len());

    if len == 0 {
        bail!(ErrorKind::NoMatch(
            input.start(),
            "lexer::unicode::decimal_number"
        ));
    }

    let split = input.split_at(len);
    Ok(ParseOutput {
        output: (input.start(), split.0.parse().unwrap()),
        remaining_input: split.1,
    })
}

// FIXME: Use Unicode UAX31-R1 instead of this simple definition
/// Parse an identifier
pub fn identifier(input: PositionedStr) -> Result<PositionedStr, String, Error> {
    let mut chars = input.chars();
    if let Some(char) = chars.next() {
        if !is_id_start(char) {
            bail!(ErrorKind::NoMatch(input.start(), "lexer::unicode::identifier"))
        }
        let idx = chars.take_while(|c| is_id_continue(*c)).count() + 1;
        let (matched, remaining) = input.split_at(idx);
        Ok(ParseOutput {
            remaining_input: remaining,
            output: (*matched).into(),
        })
    } else {
        bail!(ErrorKind::NoMatch(input.start(), "lexer::unicode::identifier"))
    }
}

fn is_id_start(ch: char) -> bool {
    (('a' as u32 <= ch as u32) && (ch as u32 <= 'z' as u32)) ||
        (('A' as u32 <= ch as u32) && (ch as u32 <= 'Z' as u32))
}

fn is_id_continue(ch: char) -> bool {
    is_id_start(ch) || ch == '_' || (('0' as u32 <= ch as u32) && (ch as u32 <= '9' as u32))
}
