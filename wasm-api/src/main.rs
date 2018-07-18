#![cfg_attr(target_arch = "wasm32", feature(proc_macro))]

extern crate nafi_parser;
extern crate ron;
#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", macro_use)]
extern crate stdweb;

#[cfg(target_arch = "wasm32")]
pub use hidden::*;
#[cfg(target_arch = "wasm32")]
mod hidden {
    use ron::ser::to_string_pretty;
    use stdweb::js_export;

    #[js_export]
    fn parse(s: &str) -> String {
        match nafi_parser::parse(s) {
            Ok(parse) => match to_string_pretty(&parse, Default::default()) {
                Ok(serialized) => serialized,
                Err(err) => format!("{}", err),
            },
            Err(err) => format!("{}", err),
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!(">.>");
}
