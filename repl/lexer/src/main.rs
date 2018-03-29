//! Read-Eval-Print-Loop for the Nafi Lexer

#![forbid(missing_debug_implementations, unconditional_recursion, future_incompatible)]
#![deny(bad_style, unsafe_code, missing_docs)]
#![warn(edition_2018, rust_2018_idioms)]

#[macro_use]
extern crate quicli;

extern crate nafi_interner;
extern crate nafi_lexer;
extern crate nafi_tokens;

use quicli::prelude::*;
use std::io;

fn repl() -> Result<()> {
    let mut buffer = String::with_capacity(80);
    let interner = nafi_interner::StringInterner::default();
    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("IO Failure");
        if buffer.trim().is_empty() {
            break;
        }
        let lexer = nafi_lexer::Lexer::new(&buffer, &interner);
        let tokens = lexer.collect::<Vec<_>>();
        nafi_tokens::dump(&tokens, io::stdout())?;
    }
    Ok(())
}

main!({
    repl()?;
});
