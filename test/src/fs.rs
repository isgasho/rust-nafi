use std::path::{Path};
use std::io::{Read, Write, BufReader, BufWriter};
use std::fs::File;

use failure::ResultExt;

use ::Result;

pub use std::fs::create_dir_all as create_dir;

/// Read file content into string
///
/// # Examples
///
/// ```rust,no_run
/// # extern crate quicli;
/// # use quicli::prelude::*;
/// # fn main() { run().unwrap() }
/// # fn run() -> Result<()> {
/// let x = read_file(".gitignore")?;
/// assert!(x.len() > 0);
/// # Ok(()) }
/// ```
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    ensure!(path.exists() && path.is_file(), "Path {:?} is not a file!", path);

    let file = File::open(path)
        .with_context(|_| format!("Could not open file {:?}", path))?;
    let mut file = BufReader::new(file);

    let mut result = String::new();
    file.read_to_string(&mut result)
        .with_context(|_| format!("Could not read file {:?}", path))?;

    Ok(result)
}

/// Write string to file
///
/// _Note:_ Replaces the current file content if the file already exists.
///
/// # Examples
///
/// ```rust,no_run
/// # extern crate quicli;
/// # use quicli::prelude::*;
/// # fn main() { run().unwrap() }
/// # fn run() -> Result<()> {
/// write_to_file("/tmp/asdasidz81zasda", "foobar")?;
/// # Ok(()) }
/// ```
pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> Result<()> {
    let path = path.as_ref();

    let file = File::create(path)
        .with_context(|_| format!("Could not create/open file {:?}", path))?;
    let mut file = BufWriter::new(file);

    file.write_all(content.as_bytes())
        .with_context(|_| format!("Could not write to file {:?}", path))?;

    Ok(())
}
