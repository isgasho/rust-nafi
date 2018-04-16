use lexer_harness::lex;

use Result;
use difference::Changeset;
use fs::*;
use rayon::prelude::*;
use std::fmt;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

const TESTCASE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/testcases");

#[derive(Debug, Fail)]
struct Failed {
    failures: Vec<TestFailure>,
}

impl fmt::Display for Failed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "There were {} failures in lexertests:",
            self.failures.len()
        )?;
        for failure in &self.failures {
            write!(f, "\n{}", failure)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum TestFailure {
    NoTarget(PathBuf),
    Mismatch(PathBuf, String, String),
}

impl fmt::Display for TestFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestFailure::NoTarget(path) => {
                write!(f, "{} did not exist, so I created it", path.display())
            },
            TestFailure::Mismatch(path, expected, actual) => {
                writeln!(
                    f,
                    "{} had a mismatch with its expected output:",
                    path.display()
                )?;
                ::format_changeset::format_changeset(f, &Changeset::new(expected, actual, "\n"))
            },
        }
    }
}

pub(crate) fn test() -> Result<()> {
    let testcases: Vec<_> = WalkDir::new(TESTCASE_DIR)
        // Taking the contents first allows me to filter to nafi files without cancelling recursion
        .contents_first(true)
        .into_iter()
        .filter_entry(|e: &DirEntry|
            e.path().extension().map(|ext| ext == "nafi").unwrap_or(false)
        )
        .filter_map(|r| r.ok())
        .collect();

    let failures: Vec<_> = testcases
        .into_par_iter()
        .map(|e: DirEntry| {
            assert!(e.file_type().is_file());
            let path_nafi = e.path();
            assert_eq!(path_nafi.extension().unwrap(), "nafi");
            let mut path_tokens = path_nafi.to_path_buf();
            path_tokens.set_extension("nafi.tokens");
            let path = path_nafi.strip_prefix(TESTCASE_DIR).unwrap().to_path_buf();

            let source = read_file(&path_nafi)
                .unwrap_or_else(
                    |e| panic!("Failed to read file {} with err {}", path.display(), e)
                )
                .replace("\r\n", "\n");
            let tokens = read_file(&path_tokens).map(
                |text| text.replace("\r\n", "\n")
            );
            (path, source, tokens)
        })
        .filter_map(|(mut path, source, tokens)| {
            let actual = lex(&source)
                .iter()
                .map(ToString::to_string)
                .map(|s| s + "\n")
                .collect();
            match tokens {
                Ok(expected) => if actual != expected {
                    Some(TestFailure::Mismatch(path, expected, actual))
                } else {
                    None
                },
                Err(_) => {
                    path.set_extension("nafi.tokens");
                    write_to_file(&path, &actual).unwrap();
                    Some(TestFailure::NoTarget(path))
                },
            }
        })
        .collect();

    if failures.is_empty() {
        Ok(())
    } else {
        Err(Failed { failures })?
    }
}
