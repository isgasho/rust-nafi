//! Read-Eval-Print-Loop for the Nafi Lexer

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, unsafe_code, missing_docs)]
#![warn(edition_2018, rust_2018_idioms)]

#[macro_use]
extern crate quicli;

extern crate nafi_lexer;
extern crate nafi_tokens;

use quicli::prelude::*;
use std::io::{self, prelude::*};

fn repl() -> Result<()> {
    let mut buffer = String::with_capacity(80);
    loop {
        buffer.clear();
        print!("? ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer).expect("IO Failure");
        if buffer.trim().is_empty() {
            break;
        }
        drive(nafi_lexer::Lexer::from(&*buffer))?;
        println!();
    }
    Ok(())
}

enum Mode {
    Code,
    String,
}

fn drive(mut lexer: nafi_lexer::Lexer) -> Result<()> {
    let mut mode = Mode::Code;
    let mut depth: Vec<usize> = vec![0];

    loop {
        match mode {
            Mode::Code => {
                use nafi_tokens::code::*;
                if let Some(tok) = lexer.next_code() {
                    println!("{}{}", " ".repeat(depth.len() - 1), tok);
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
                use nafi_tokens::string::*;
                if let Some(tok) = lexer.next_string() {
                    println!("{}{}", " ".repeat(depth.len() - 1), tok);
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
                }
            },
        }
    }

    Ok(())
}

main!({
    repl()?;
});
