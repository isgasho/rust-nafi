use nnom::{ParseOutput, ParseResult};
use nnom::slice::PositionedStr;
use std::sync::Arc;
use str_intern::interned;
use tokens::BigUint;

pub fn is_newline(ch: char) -> bool { matches!(ch as u32, 0xA..=0xD | 0x85 | 0x2028 | 0x2029) }

pub fn white_space(input: PositionedStr) -> ParseResult<PositionedStr, PositionedStr, ()> {
    let idx = input
        .char_indices()
        .filter(|&(_, ch)| !ch.is_whitespace())
        .map(|(idx, _)| idx)
        .next()
        .unwrap_or_else(|| input.len());
    if idx != 0 {
        let (output, remaining_input) = input.split_at(idx);
        Ok(ParseOutput {
            remaining_input,
            output,
        })
    } else {
        Err(())
    }
}

pub fn identifier(input: PositionedStr) -> ParseResult<PositionedStr, Arc<str>, ()> {
    // TODO: Use Unicode UAX31-R1 instead of this simple definition
    let mut chars = input.chars();
    if let Some(ch) = chars.next() {
        if !ch.is_alphabetic() {
            return Err(());
        }
        let idx = chars.take_while(|ch| ch.is_alphanumeric()).count() + 1;
        let (matched, remaining_input) = input.split_at(idx);
        Ok(ParseOutput {
            remaining_input,
            output: interned(&matched),
        })
    } else {
        Err(())
    }
}

pub fn decimal_number(input: PositionedStr) -> ParseResult<PositionedStr, BigUint, ()> {
    let idx = input.chars().take_while(|ch| ch.is_digit(10)).count();
    if idx != 0 {
        let (matched, remaining_input) = input.split_at(idx);
        Ok(ParseOutput {
            remaining_input,
            output: matched.parse().unwrap(),
        })
    } else {
        Err(())
    }
}
