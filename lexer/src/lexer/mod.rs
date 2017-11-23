use nnom::{ParseOutput, ParseResult};
use nnom::combinators::many0;
use nnom::slice::PositionedStr;
use tokens::{Keyword, Symbol, Token};
use unic_ucd_category::GeneralCategory;

mod literals;
mod unicode;
mod whitespace;

use self::literals::{integer_literal, string_literal};
use self::unicode::identifier;
use self::whitespace::whitespace;

pub fn tokens(input: PositionedStr) -> ParseResult<(), Vec<Token>, !> {
    many0(token)(input).map(
        |ParseOutput {
             remaining_input,
             output,
         }| {
            assert!(remaining_input.is_empty());
            ParseOutput {
                remaining_input: (),
                output,
            }
        },
    )
}

fn token(input: PositionedStr) -> ParseResult<PositionedStr, Token, ()> {
    Err(())
        .or_else(|_| whitespace(input))
        .or_else(|_| integer_literal(input))
        .or_else(|_| string_literal(input))
        .or_else(|_| identifier_like(input))
        .or_else(|_| symbol(input))
        .or_else(|_| _unknown(input))
}

/// `Token::Identifier` or `Token::Keyword`
fn identifier_like(input: PositionedStr) -> ParseResult<PositionedStr, Token, ()> {
    identifier(input).map(
        |ParseOutput {
             remaining_input,
             output,
         }| {
            ParseOutput {
                remaining_input,
                output: match &*output {
                    "let" => Token::Keyword(input.start(), Keyword::Let),
                    "mutable" => Token::Keyword(input.start(), Keyword::Mutable),
                    "if" => Token::Keyword(input.start(), Keyword::If),
                    "else" => Token::Keyword(input.start(), Keyword::Else),
                    _ => Token::Identifier(input.start(), output),
                },
            }
        },
    )
}

/// `Token::Symbol`
fn symbol(input: PositionedStr) -> ParseResult<PositionedStr, Token, ()> {
    let ch = input.chars().next().ok_or(())?;
    let category = GeneralCategory::of(ch);
    if category.is_symbol() || category.is_punctuation() {
        Ok(ParseOutput {
            remaining_input: input.split_at(ch.len_utf8()).1,
            output: Token::Symbol(input.start(), Symbol::from(ch)),
        })
    } else {
        Err(())
    }
}

/// `Token::_Unknown`
fn _unknown(input: PositionedStr) -> ParseResult<PositionedStr, Token, ()> {
    let ch = input.chars().next().ok_or(())?;
    Ok(ParseOutput {
        remaining_input: input.split_at(ch.len_utf8()).1,
        output: Token::_Unknown(input.start(), ch),
    })
}
