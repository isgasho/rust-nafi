#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r#"(?m)^([0-9A-F]{4,6})(?:..([0-9A-F]{4,6}))?\s*;"#).unwrap();
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn process(from: &Path, to: &Path) -> Result<(), Box<Error>> {
    if to.exists() {
        return Ok(());
    }

    let txt = {
        let mut txt = String::new();
        File::open(from)?.read_to_string(&mut txt)?;
        txt
    };
    let mut out = File::create(to)?;

    for pat in RE.captures_iter(&txt) {
        write!(out, "\\x{{{}}}", &pat[1])?;
        if let Some(mat) = pat.get(2) {
            write!(out, "-\\x{{{}}}", mat.as_str())?;
        }
    }

    Ok(())
}

fn main() {
    process(
        &Path::new("resources/xid_continue.txt"),
        &Path::new("resources/xid_continue.regex"),
    ).unwrap();
    process(
        &Path::new("resources/xid_start.txt"),
        &Path::new("resources/xid_start.regex"),
    ).unwrap();
    process(
        &Path::new("resources/white_space.txt"),
        &Path::new("resources/white_space.regex"),
    ).unwrap();
}
