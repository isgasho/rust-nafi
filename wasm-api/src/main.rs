#[cfg(target_arch = "wasm32")]
pub use self::hidden::*;
#[cfg(target_arch = "wasm32")]
mod hidden {
    use ron::ser::to_string_pretty;
    use stdweb::{__js_raw_asm, js_export};

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
