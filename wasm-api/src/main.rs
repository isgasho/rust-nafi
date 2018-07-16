#![cfg_attr(target_arch = "wasm32", feature(proc_macro))]

extern crate nafi_parser;
extern crate ron;
#[cfg_attr(target_arch = "wasm32", macro_use)]
extern crate stdweb;

use ron::ser::to_string_pretty;
#[cfg(target_arch = "wasm32")]
use stdweb::js_export;

#[cfg_attr(target_arch = "wasm32", js_export)]
#[cfg_attr(not(target_arch = "wasm32"), allow(unused))]
fn parse(s: &str) -> String {
    match nafi_parser::parse(s) {
        Ok(parse) => match to_string_pretty(&parse, Default::default()) {
            Ok(serialized) => serialized,
            Err(err) => format!("{}", err),
        },
        Err(err) => format!("{}", err),
    }
}

fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}
