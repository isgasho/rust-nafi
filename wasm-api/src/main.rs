#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;
extern crate nafi_lexer_repl;

#[cfg(target_arch = "wasm32")]
mod hide {
    use super::*;
    use stdweb::js_export;
    use std::fmt::Write;

    #[js_export]
    pub fn lex(input: &str) -> String {
        let tokens = nafi_lexer_repl::lex(input);
        let mut out = String::new();
        for tok in tokens {
            writeln!(out, "{}", tok).unwrap();
        }
        out
    }

    pub fn main() {
        stdweb::initialize();
        stdweb::event_loop();
    }
}

#[cfg(target_arch = "wasm32")]
pub use hide::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    hide::main()
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("nafi-wasm-api is only useful on wasm32")
}
