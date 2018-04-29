#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;
extern crate nafi_lexer_repl;

use stdweb::js_export;
use std::fmt::Write;

#[js_export]
fn lex(input: &str) -> String {
    let tokens = nafi_lexer_repl::lex(input);
    let mut out = String::new();
    for tok in tokens {
        writeln!(out, "{}", tok).unwrap();
    }
    out
}

fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}
