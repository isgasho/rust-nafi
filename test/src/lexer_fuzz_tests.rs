use lexer::Lexer;

use Result;
use rayon::prelude::*;
use std::char;

pub(crate) fn test() -> Result<()> {
    (0..(char::MAX as u32 + 1))
        .into_par_iter()
        .filter_map(char::from_u32)
        .filter_map(|ch| {
            let s = &ch.to_string();
            let code = Lexer::new(s).next_code().is_none();
            let string = Lexer::new(s).next_string().is_none();
            if code || string {
                Some((ch, code, string))
            } else {
                None
            }
        })
        .for_each(drop);
    // Failure ==> panic
    Ok(())
}
