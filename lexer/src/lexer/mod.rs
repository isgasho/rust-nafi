use tokens::Token;

mod literals;
mod unicode;
mod whitespace;

use error::*;
use lexer::literals::{integer_literal, string_literal};
use lexer::whitespace::whitespace;
use nnom::prelude::{ParseOutput, PositionedStr, Result, many0};

/// Vec<Token>
pub fn tokens(input: PositionedStr) -> Result<(), Vec<Token>, !> {
    many0(token)(input).map(|ParseOutput { remaining_input, output }| {
        assert!(remaining_input.is_empty());
        ParseOutput { remaining_input: (), output }
    })
}

/// Token
fn token(input: PositionedStr) -> Result<PositionedStr, Token, Error> {
    integer_literal(input)
        .or_else(|_| string_literal(input))
        .or_else(|_| whitespace(input))
        .or_else(|_| _unknown(input))
        .map_err(|e| {
            e.chain_err(|| ErrorKind::NoMatch(input.start(), "lexer::token"))
        })
}

/// Token::_Unknown
fn _unknown(input: PositionedStr) -> Result<PositionedStr, Token, Error> {
    if let Some(first_char) = input.chars().next() {
        Ok(ParseOutput {
            remaining_input: input.split_at(first_char.len_utf8()).1,
            output: Token::_Unknown(input.start()),
        })
    } else {
        Err(ErrorKind::NoMatch(input.start(), "lexer::_unknown").into())
    }
}
