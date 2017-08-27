use std::{char, u32};

use error::*;
use lexer::unicode::decimal_number;
use nnom::prelude::{ParseOutput, PositionedStr, Result};
use tokens::{StringFragments, Token};

/// Token::IntegerLiteral
pub fn integer_literal(input: PositionedStr) -> Result<PositionedStr, Token, Error> {
    decimal_number(input)
        .map(
            |ParseOutput {
                 output: (pos, integer),
                 remaining_input,
             }| {
                ParseOutput {
                    output: Token::IntegerLiteral(pos, integer),
                    remaining_input,
                }
            },
        )
        .chain_err(|| {
            ErrorKind::NoMatch(input.start(), "lexer::literals::integer_literal")
        })
}

// NOTE: Allow other quotation marks <https://unicode-table.com/en/sets/quotation-marks/> ?
// NOTE: Other quotation marks might be used as special string-like literals
// FIXME: Monolith with i, _i, and __i
/// Token::StringLiteral
pub fn string_literal(input: PositionedStr) -> Result<PositionedStr, Token, Error> {
    if !input.starts_with("\"") {
        bail!(ErrorKind::NoMatch(
            input.start(),
            "lexer::literals::string_literal"
        ));
    }

    let pos = input.start();
    let mut remaining_input = input.split_at(1).1;
    let mut string = StringFragments::new();

    loop {
        match remaining_input.chars().next() {
            Some('\"') => {
                remaining_input = remaining_input.split_at(1).1;
                break;
            },
            Some('\\') => {
                let simple_escape_len = remaining_input.chars().take(2).map(char::len_utf8).sum();
                let (escape, rest) = remaining_input.split_at(simple_escape_len);
                remaining_input = rest;
                match &*escape {
                    "\\\\" => string.push('\\'),
                    "\\\"" => string.push('\"'),
                    "\\n" => string.push('\n'),
                    "\\t" => string.push('\t'),
                    "\\u" => unimplemented!(), // FIXME
                    s @ _ => string.push_invalid_escape(&s[1..]),
                }
            },
            Some(_) => {
                let first_interesting_index = remaining_input
                    .find(|c| c == '\"' || c == '\\')
                    .unwrap_or(remaining_input.len());
                let (uninteresting, rest) = remaining_input.split_at(first_interesting_index);
                string.push_str(&*uninteresting);
                remaining_input = rest;
            },
            None => break,
        }
    }

    Ok(ParseOutput {
        remaining_input,
        output: Token::StringLiteral(pos, string),
    })

    //IResult::Done(mut __i, o) if o == "u" => {
    //    // FIXME: So many bad names and magic numbers
    //    // FIXME: This should be a separate parser bit (string between {})
    //    if __i.chars().next() == Some('{') {
    //        if let Some(idx) = __i.find('}') {
    //            let codepoint = &__i[1..idx];
    //            i = &__i[idx + 1..];
    //            if let Ok(Some(char)) =
    //                u32::from_str_radix(codepoint, 16).map(char::from_u32)
    //            {
    //                string.push(char);
    //            } else {
    //                string.push_invalid_escape(&_i[..idx + 2]);
    //            }
    //        } else {
    //            string.push_invalid_escape(_i);
    //            i = "";
    //        }
    //    } else {
    //        string.push_invalid_escape("u");
    //        i = __i;
    //    }
    //},
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn string_escapes() {
//        assert_eq!(
//            string_literal("\"\\\"\""), // "\""
//            IResult::Done("", Token::StringLiteral("\"".into()))
//        );
//        assert_eq!(
//            string_literal("\"\\n\""), // "\n"
//            IResult::Done("", Token::StringLiteral("\n".into()))
//        );
//        assert_eq!(
//            string_literal("\"\\t\""), // "\t"
//            IResult::Done("", Token::StringLiteral("\t".into()))
//        );
//        assert_eq!(
//            string_literal("\"\\u{FFFF}\""), // "\u{FFFF}"
//            IResult::Done("", Token::StringLiteral("\u{FFFF}".into()))
//        );
//    }
//
//    #[test]
//    fn invalid_string_escapes() {
//        assert_eq!(
//            string_literal("\"\\u{}\""),
//            IResult::Done(
//                "",
//                Token::StringLiteral({
//                    let mut fragments = StringFragments::new();
//                    fragments.push_invalid_escape("u{}");
//                    fragments
//                })
//            )
//        )
//    }
//}
