#[macro_use]
extern crate quicli;
#[macro_use]
extern crate failure;
extern crate walkdir;
extern crate difference;
extern crate ansi_term;

extern crate nafi_lexer_repl as lexer;

use quicli::prelude::*;

mod lexertests;
mod format_changeset;

/// Test harness for structural tests of the rust-nafi project
#[derive(Debug, StructOpt)]
struct Opts {
    // TODO
}

fn test(_opts: Opts) -> Result<()> {
    lexertests::test()?;
    println!("lexertests were successful!");
    Ok(())
}

main!(|opts: Opts| {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap_or_else(|_| eprintln!("Color support unavailable"));
    match test(opts) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
});
