use Span;
use lexer::token;
use lexer::unicode::decimal_number;
use nom::{self, IResult, InputLength, Slice};
use std::{char, u32};
use tokens::{StringFragments, Symbol, Token};

/// `Token::Literal(Literal::Integer)`
pub fn integer_literal(i: Span) -> IResult<Span, Token> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    do_parse!(i,
        pos: position!() >>
        o: call!(decimal_number) >>
        (Token::Literal(pos.offset, o.into()))
    )
}

/// `Token::Literal(Literal::String)`
pub fn string_literal(i: Span) -> IResult<Span, Token> {
    // TODO: Do the whole transformation with Span rather than &str
    let mut slice = i.fragment;

    if !slice.starts_with('"') {
        return Err(nom::Err::Error(error_position!(i, nom::ErrorKind::Custom(0))));
    }

    slice = &slice[1..];
    let mut string = StringFragments::new();

    loop {
        match slice.chars().next() {
            Some('\"') => {
                slice = &slice[1..];
                break;
            },
            Some('\\') => {
                slice = &slice[1..];
                match slice.chars().next() {
                    Some('"') => {
                        string.push_char('"');
                        slice = &slice[1..];
                    },
                    Some('\'') => {
                        string.push_char('\'');
                        slice = &slice[1..];
                    },
                    Some('\\') => {
                        string.push_char('\\');
                        slice = &slice[1..];
                    },
                    Some('r') => {
                        string.push_char('\r');
                        slice = &slice[1..];
                    },
                    Some('n') => {
                        string.push_char('\n');
                        slice = &slice[1..];
                    },
                    Some('t') => {
                        string.push_char('\t');
                        slice = &slice[1..];
                    },
                    Some('u') => {
                        // We can't slice off the `u` yet because if we get an escape of
                        // `\u{invalid}` the invalid escape slice contains the `u`
                        if slice.starts_with("u{") {
                            let idx = slice.find('}').unwrap_or_else(|| slice.len());
                            // Allow EOF to close to avoid eating string as other tokens
                            let codepoint = u32::from_str_radix(&slice[2..idx], 16);
                            if let Ok(Some(ch)) = codepoint.map(char::from_u32) {
                                string.push_char(ch);
                            } else {
                                string.push_invalid_escape(&slice[..idx + 1])
                            }
                            slice = &slice[idx + 2..];
                        } else {
                            string.push_invalid_escape('u'.to_string());
                            slice = &slice[1..];
                        }
                    },
                    Some('{') => {
                        slice = &slice[1..];
                        let mut tokens: Vec<Token> = vec![];
                        let mut depth = 1u32;
                        while let Ok((tail, o)) = token(Span {
                            // TODO: maintain span to maintain offset/line info
                            offset: 0,
                            line: 0,
                            fragment: slice,
                        }) {
                            slice = tail.fragment;
                            match o {
                                Token::Symbol(_, Symbol::LeftCurlyBracket) => depth += 1,
                                Token::Symbol(_, Symbol::RightCurlyBracket) => {
                                    depth -= 1;
                                    if depth == 0 {
                                        break;
                                    }
                                },
                                _ => {},
                            }
                            tokens.push(o);
                        }
                        string.push_interpolation(tokens);
                    },
                    Some(c) => {
                        string.push_invalid_escape(c.to_string());
                        slice = &slice[c.len_utf8()..];
                    },
                    None => break, // Allow EOF to close to avoid eating string as other tokens
                }
            },
            Some(_) => {
                let idx = slice
                    .find(|ch| ch == '"' || ch == '\\')
                    .unwrap_or_else(|| slice.len()); // Allow EOF to close
                string.push_string(&slice[..idx]);
                slice = &slice[idx..];
            },
            None => break, // Allow EOF to close to avoid eating string as other tokens
        }
    }

    Ok((
        i.slice(i.input_len() - slice.len()..),
        Token::Literal(i.offset, string.into()),
    ))
}
