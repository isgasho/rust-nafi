//! Unicode adapters for nom parsers

use nom::{self, IResult};

// FIXME: match on Pattern_White_Space
/// Recognize one or more unicode whitespaces
pub fn white_space(input: &str) -> IResult<&str, &str> {
    for (idx, char) in input.char_indices() {
        if !(char == ' ' || char == '\t' || char == '\r' || char == '\n') {
            if idx == 0 {
                return IResult::Error(error_position!(nom::ErrorKind::MultiSpace, input));
            } else {
                return IResult::Done(&input[idx..], &input[..idx]);
            }
        }
    }
    return IResult::Done("", input)
}

// FIXME: match on Numeric_Type=Decimal
/// Parse a decimal number
pub fn decimal_number(input: &str) -> IResult<&str, u64> {
    let len = input
        .char_indices()
        .find(|&(_, char)| !char.is_digit(10))
        .map(|(idx, _)| idx)
        .unwrap_or(input.len());

    if len == 0 {
        IResult::Error(error_position!(nom::ErrorKind::Digit, input))
    } else {
        match input[..len].parse::<u64>() {
            Ok(num) => IResult::Done(&input[len..], num),
            Err(_) => {
                eprintln!(
                    "Decimal number `{}` is too big. Expect strange lexer results.",
                    &input[..len]
                );
                IResult::Error(error_position!(nom::ErrorKind::MapRes, input))
            },
        }
    }
}
