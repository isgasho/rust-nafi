use Span;
use lexer::token;
use lexer::unicode::decimal_number;
use nom::{IResult, InputLength, Slice};
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

// TODO: Consider returning `Err(nom::Err::Incomplete(nom::Needed))` instead of closing with EOF
// TODO: Consider returning `Err(nom::Err::Failure(nom::Context))` instead of above options
/// `Token::Literal(Literal::String)`
pub fn string_literal(mut i: Span) -> IResult<Span, Token> {
    let pos = i.offset;

    tag!(i, "\"")?;

    i = i.slice(1..);
    let mut string = StringFragments::new();

    loop {
        match i.fragment.chars().next() {
            Some('\"') => {
                i = i.slice(1..);
                break;
            },
            Some('\\') => {
                i = i.slice(1..);
                macro_rules! push {
                    ($to:expr) => {{
                        string.push_char($to);
                        i = i.slice(1..);
                    }};
                }
                match i.fragment.chars().next() {
                    Some('\"') => push!('\"'),
                    Some('\'') => push!('\''),
                    Some('\\') => push!('\\'),
                    Some('r') => push!('\r'),
                    Some('n') => push!('\n'),
                    Some('t') => push!('\t'),
                    Some('u') => {
                        // We can't slice off the `u` yet because if we get an escape of
                        // `\u{invalid}` the invalid escape slice contains the `u`
                        if i.fragment.starts_with("u{") {
                            let idx = i.fragment.find('}').unwrap_or_else(|| i.input_len());
                            // Allow EOF to close to avoid eating string as other tokens
                            let codepoint = u32::from_str_radix(&i.fragment[2..idx], 16);
                            if let Ok(Some(ch)) = codepoint.map(char::from_u32) {
                                string.push_char(ch);
                            } else {
                                string.push_invalid_escape(&i.fragment[..idx + 1])
                            }
                            i = i.slice(idx + 2..);
                        } else {
                            string.push_invalid_escape('u'.to_string());
                            i = i.slice(1..);
                        }
                    },
                    Some('{') => {
                        i = i.slice(1..);
                        let mut tokens: Vec<Token> = vec![];
                        let mut depth = 1u32;
                        while let Ok((tail, o)) = token(i) {
                            i = tail;
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
                        i = i.slice(c.len_utf8()..);
                    },
                    None => break, // Allow EOF to close to avoid eating string as other tokens
                }
            },
            Some(_) => {
                let idx = i.fragment
                    .find(|ch| ch == '"' || ch == '\\')
                    .unwrap_or_else(|| i.fragment.len()); // Allow EOF to close
                string.push_string(&i.fragment[..idx]);
                i = i.slice(idx..);
            },
            None => break, // Allow EOF to close to avoid eating string as other tokens
        }
    }

    Ok((i, Token::Literal(pos, string.into())))
}
