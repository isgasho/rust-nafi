use super::unicode::{is_newline, white_space};
use nnom::{ParseOutput, ParseResult};
use nnom::slice::PositionedStr;
use tokens::Token;

/// `Token::Whitespace`
pub fn whitespace(input: PositionedStr) -> ParseResult<PositionedStr, Token, ()> {
    let mut rest = input;
    while let Ok(ParseOutput {
        remaining_input, ..
    }) = some_whitespace(rest)
    {
        rest = remaining_input;
    }
    if rest.start() != input.start() {
        Ok(ParseOutput {
            remaining_input: rest,
            output: Token::Whitespace(input.start()),
        })
    } else {
        Err(())
    }
}

/// Consume some whitespace.
fn some_whitespace(input: PositionedStr) -> ParseResult<PositionedStr, PositionedStr, ()> {
    Err(())
        .or_else(|_| white_space(input))
        .or_else(|_| line_comment(input))
        .or_else(|_| block_comment(input))
}

/// Parse a line comment
fn line_comment(input: PositionedStr) -> ParseResult<PositionedStr, PositionedStr, ()> {
    if !input.starts_with("//") {
        return Err(());
    }

    let idx = input.find(is_newline).unwrap_or_else(|| input.len());
    let (output, remaining_input) = input.split_at(idx);
    Ok(ParseOutput {
        remaining_input,
        output,
    })
}

/// Parse a block comment
fn block_comment(input: PositionedStr) -> ParseResult<PositionedStr, PositionedStr, ()> {
    if !input.starts_with("/*") {
        return Err(());
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

    let (output, remaining_input) = input.split_at(idx);
    Ok(ParseOutput {
        remaining_input,
        output,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn block_comment() {
        assert_eq!(
            super::block_comment("/** /* */*/".into()),
            Ok(ParseOutput {
                output: "/** /* */*/".into(),
                remaining_input: PositionedStr::new("", 11),
            })
        );
    }

    #[test]
    fn line_comment() {
        assert_eq!(
            super::line_comment("// any amount of text you want".into()),
            Ok(ParseOutput {
                output: "// any amount of text you want".into(),
                remaining_input: PositionedStr::new("", 30),
            })
        );
    }
}
