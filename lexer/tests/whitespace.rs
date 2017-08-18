//! Test sequences of non-significant whitespace & comments get collapsed to one Token::_Whitespace

extern crate nafi_lexer;
use nafi_lexer::{lex, Token};

fn whitespace() -> Vec<Token> { vec![Token::_Whitespace] }

#[test]
fn ascii_white_space() {
    assert_eq!(lex(" "), whitespace());
    assert_eq!(lex("\t"), whitespace());
    assert_eq!(lex("\n"), whitespace());
    assert_eq!(lex("\r\n"), whitespace());
    assert_eq!(
        // <https://en.wikipedia.org/wiki/Whitespace_(programming_language)#Sample_code> (excerpt)
        lex("   \t  \t\n\t\n     \t\t  \t \t\n\t\n     \t\t \t\t  \n\t\n"),
        whitespace()
    )
}

#[test]
#[should_panic(expected = "_Unknown")]
fn pattern_white_space() {
    assert_eq!(lex("\u{0009}"), whitespace());
    assert_eq!(lex("\u{000A}"), whitespace());
    assert_eq!(lex("\u{000B}"), whitespace());
    assert_eq!(lex("\u{000C}"), whitespace());
    assert_eq!(lex("\u{000D}"), whitespace());
    assert_eq!(lex("\u{0020}"), whitespace());
    assert_eq!(lex("\u{0085}"), whitespace());
    assert_eq!(lex("\u{200E}"), whitespace());
    assert_eq!(lex("\u{200F}"), whitespace());
    assert_eq!(lex("\u{2028}"), whitespace());
    assert_eq!(lex("\u{2029}"), whitespace());
}

#[test]
fn line_comment() {
    assert_eq!(lex("//"), whitespace());
    assert_eq!(lex("// this is a comment"), whitespace());
    assert_eq!(lex("  \t  \t  // this is a comment"), whitespace());
    assert_eq!(
        whitespace(),
        lex(
            "\
// Haikus are easy
// But sometimes they don't make sense
// Refrigerator
"
        )
    )
}

#[test]
fn block_comment() {
    assert_eq!(lex("/**/"), whitespace());
    assert_eq!(lex("/* this is a block comment */"), whitespace());
    assert_eq!(
        lex("/* block comments /* can be */ nested! */"),
        whitespace()
    );
    assert_eq!(
        lex("/* block comments /* get closed /* by eof"),
        whitespace()
    )
}
