//! Read-Eval-Print-Loop for the Nafi Lexer

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, unsafe_code, missing_docs)]
#![warn(edition_2018, rust_2018_idioms)]

extern crate failure;

extern crate nafi_lexer_repl as lexer;

use failure::Error;
use std::io::{self, prelude::*};

fn repl() -> Result<(), Error> {
    let mut buffer = String::with_capacity(80);
    loop {
        buffer.clear();
        print!("? ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer).expect("IO Failure");
        if buffer.trim().is_empty() {
            break;
        }
        for tok in lexer::lex(&buffer) {
            println!("{}", tok);
        }
        println!();
    }
    Ok(())
}

fn main() { repl().unwrap(); }
