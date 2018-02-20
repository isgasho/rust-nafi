#[macro_use]
extern crate quicli;

extern crate nafi_interner;
extern crate nafi_lexer;
extern crate nafi_tokens;

use quicli::prelude::*;
use std::io;

fn repl() -> Result<()> {
    let mut buffer = String::with_capacity(80);
    let mut interner = nafi_interner::StringInterner::default();
    Ok(loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("IO Failure");
        if buffer.trim().is_empty() {
            break;
        }
        let lexer = nafi_lexer::Lexer::new(&buffer, &mut interner);
        let tokens = lexer.collect::<Vec<_>>();
        nafi_tokens::dump(&tokens, io::stdout())?;
    })
}

main!({
    repl()?;
});
