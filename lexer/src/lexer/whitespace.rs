use error::*;
use lexer::unicode::{is_newline_char, white_space};
use nnom::prelude::{ParseOutput, PositionedStr, Result};
use tokens::Token;

/// Token::Whitespace
pub fn whitespace(input: PositionedStr) -> Result<PositionedStr, Token, Error> {
    some_whitespace(input)
        .map(|ParseOutput { remaining_input: mut rest, .. }| {
            while let Ok(ParseOutput { remaining_input, .. }) = some_whitespace(rest) {
                rest = remaining_input;
            }
            ParseOutput {
                remaining_input: rest,
                output: Token::Whitespace(input.start()),
            }
        })
        .chain_err(|| {
            ErrorKind::NoMatch(input.start(), "lexer::whitespace::whitespace")
        })
}

/// Consume some whitespace.
fn some_whitespace(input: PositionedStr) -> Result<PositionedStr, PositionedStr, Error> {
    // FIXME: chain errors!
    white_space(input)
        .or_else(|_| line_comment(input))
        .or_else(|_| block_comment(input))
        .map_err(|_| {
            ErrorKind::NoMatch(input.start(), "lexer::whitespace::some_whitespace").into()
        })
}

/// Parse a line comment
fn line_comment(input: PositionedStr) -> Result<PositionedStr, PositionedStr, Error> {
    if !input.starts_with("//") {
        bail!(ErrorKind::NoMatch(
            input.start(),
            "lexer::whitespace::line_comment"
        ));
    }

    let comment_end_idx = input.find(is_newline_char).unwrap_or(input.len());
    let split = input.split_at(comment_end_idx);
    Ok(ParseOutput {
        output: split.0,
        remaining_input: split.1,
    })
}

/// Parse a block comment
fn block_comment(input: PositionedStr) -> Result<PositionedStr, PositionedStr, Error> {
    if !input.starts_with("/*") {
        bail!(ErrorKind::NoMatch(
            input.start(),
            "lexer::whitespace::block_comment"
        ));
    }

    let mut idx: usize = 2;
    let mut depth = 1;

    while depth > 0 && idx < input.len() {
        let remaining_input = &input[idx..];
        if remaining_input.starts_with("/*") {
            depth += 1;
            idx += 2;
        } else if remaining_input.starts_with("*/") {
            depth -= 1;
            idx += 2;
        } else {
            idx += remaining_input.chars().next().unwrap().len_utf8();
        }
    }

    let split = input.split_at(idx);
    Ok(ParseOutput {
        output: split.0,
        remaining_input: split.1,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn block_comment() {
        assert_eq!(
            super::block_comment("/** /* */*/".into()).map_err(|e| e.to_string()),
            Ok(ParseOutput {
                output: "/** /* */*/".into(),
                remaining_input: PositionedStr::new("", 11),
            })
        );
    }

    #[test]
    fn line_comment() {
        assert_eq!(
            super::line_comment("// any amount of text you want".into()).map_err(|e| e.to_string()),
            Ok(ParseOutput {
                output: "// any amount of text you want".into(),
                remaining_input: PositionedStr::new("", 30),
            })
        );
    }
}
