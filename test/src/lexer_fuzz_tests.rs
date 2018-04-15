use lexer::Lexer;

use quicli::prelude::*;

#[derive(Debug, Default, Fail)]
#[fail(display = "Lexer halted when given some characters: CODE mode {:?}; STRING mode {:?}",
       string, code)]
struct Failed {
    code: Vec<char>,
    string: Vec<char>,
}

impl Failed {
    fn is_failure(&self) -> bool {
        !self.code.is_empty() || !self.string.is_empty()
    }
}

pub(crate) fn test() -> Result<()> {
    let mut failed = Failed::default();

    for ch in chars!(..) {
        let s = ch.to_string();

        if Lexer::new(&s).next_code().is_none() {
            failed.code.push(ch);
        }
        if Lexer::new(&s).next_string().is_none() {
            failed.string.push(ch);
        }
    }

    if failed.is_failure() {
        Err(failed)?
    }
    Ok(())
}
