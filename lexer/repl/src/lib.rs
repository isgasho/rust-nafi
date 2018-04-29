extern crate nafi_lexer as lexer;
extern crate nafi_tokens as tokens;
use std::fmt;

enum Mode {
    Code,
    String,
}

pub enum Token<'a> {
    Code(usize, tokens::code::Token<'a>),
    String(usize, tokens::string::Token<'a>),
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Code(depth, tok) => write!(f, "{}{}", " ".repeat(depth), tok),
            Token::String(depth, tok) => write!(f, "{}{}", " ".repeat(depth), tok),
        }
    }
}

pub fn lex(source: &str) -> Vec<Token> {
    let mut lexer = lexer::Lexer::from(source);
    let mut out = vec![];
    let mut mode = Mode::Code;
    let mut depth = vec![0u32];

    loop {
        match mode {
            Mode::Code => {
                use tokens::code::*;
                if let Some(tok) = lexer.next_code() {
                    out.push(::Token::Code(depth.len() - 1, tok));
                    match tok {
                        Token {
                            kind: Kind::Symbol,
                            source: "}",
                            ..
                        } => {
                            let d = depth.pop().unwrap();
                            if d == 0 && !depth.is_empty() {
                                mode = Mode::String;
                            } else {
                                depth.push(d.saturating_sub(1));
                            }
                        },
                        Token {
                            kind: Kind::Symbol,
                            source: "{",
                            ..
                        } => {
                            let d = depth.pop().unwrap();
                            depth.push(d + 1);
                        },
                        Token {
                            kind: Kind::LiteralStringStart,
                            ..
                        } => {
                            depth.push(0);
                            mode = Mode::String;
                        },
                        _ => {},
                    }
                } else {
                    break;
                }
            },
            Mode::String => {
                use tokens::string::*;
                if let Some(tok) = lexer.next_string() {
                    out.push(::Token::String(depth.len() - 1, tok));
                    match tok {
                        Token {
                            kind: Kind::InterpolationStart,
                            ..
                        } => {
                            depth.push(0);
                            mode = Mode::Code;
                        },
                        Token {
                            kind: Kind::StringEnd,
                            ..
                        } => {
                            depth.pop().unwrap();
                            mode = Mode::Code;
                        },
                        _ => {},
                    }
                } else {
                    break;
                }
            },
        }
    }

    out
}
