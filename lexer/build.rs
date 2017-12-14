#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"(?m)^([0-9A-F]{4,6})(?:..([0-9A-F]{4,6}))?\s*;"#).unwrap();
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn process(from: &Path, to: &Path) {
    if to.exists() {
        return;
    }

    let txt = {
        let mut txt = String::new();
        File::open(from).unwrap().read_to_string(&mut txt).unwrap();
        txt
    };
    let mut out = File::create(to).unwrap();

    out.write(b"[").unwrap();
    for pat in RE.captures_iter(&txt) {
        out.write(b"\\x{").unwrap();
        out.write(pat[1].as_bytes()).unwrap();
        out.write(b"}").unwrap();
        if let Some(mat) = pat.get(2) {
            out.write(b"-\\x{").unwrap();
            out.write(mat.as_str().as_bytes()).unwrap();
            out.write(b"}").unwrap();
        }
    }
    out.write(b"]").unwrap();
}

fn main() {
    process(
        &Path::new("src/lexer/xid_continue.txt"),
        &Path::new("src/lexer/xid_continue.regex"),
    );
    process(
        &Path::new("src/lexer/xid_start.txt"),
        &Path::new("src/lexer/xid_start.regex"),
    );
}
