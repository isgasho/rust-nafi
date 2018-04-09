use lexer;

use std::fmt;
use difference::Changeset;
use quicli::prelude::*;
use walkdir::WalkDir;

const TESTCASE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/testcases");

#[derive(Debug, Fail)]
struct Failed {
    failures: Vec<TestFailure>,
}

impl fmt::Display for Failed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "There were {} failures in lexertests:\n", self.failures.len())?;
        for failure in &self.failures {
            writeln!(f, "{}", failure)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum TestFailure {
    NoTarget(String),
    Mismatch(String, String, String),
}

impl fmt::Display for TestFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestFailure::NoTarget(test) => write!(f,
                "{} did not exist, so I created it",
                test
            ),
            TestFailure::Mismatch(test, expected, actual) => {
                writeln!(f, "{} had a mismatch with its expected output:", test)?;
                ::format_changeset::format_changeset(f, &Changeset::new(expected, actual, "\n"))
            }
        }
    }
}

pub(crate) fn test() -> Result<()> {
    let mut failures = vec![];

    for entry in WalkDir::new(TESTCASE_DIR) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let nafi_path = entry.path();
        if nafi_path.extension().map(|ext| ext != "nafi").unwrap_or(true) {
            continue;
        }

        let mut tokens_path = nafi_path.to_path_buf();
        tokens_path.set_extension("nafi.tokens");

        let source = read_file(&nafi_path)?.replace("\r\n", "\n");
        let expected = read_file(&tokens_path).map(|text| text.replace("\r\n", "\n"));
        let actual = lexer::lex(&source).iter().map(ToString::to_string).map(|s| s + "\n").collect();

        match expected {
            Ok(expected) => if actual != expected {
                failures.push(TestFailure::Mismatch(
                    nafi_path.strip_prefix(TESTCASE_DIR)?.to_string_lossy().into_owned(),
                    expected, actual
                ))
            }
            Err(_) => {
                write_to_file(&tokens_path, &actual)?;
                failures.push(TestFailure::NoTarget(
                    tokens_path.strip_prefix(TESTCASE_DIR)?.to_string_lossy().into_owned(),
                ))
            }
        }
    }

    if failures.is_empty() {
        Ok(())
    } else {
        Err(Failed { failures })?
    }
}
