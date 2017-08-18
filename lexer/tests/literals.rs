//! Test literals in NAFI source code, e.g. numbers, strings

extern crate nafi_lexer;
use nafi_lexer::{lex, Token};

use std::u64;

fn literal_u64(int: u64) -> Vec<Token> { vec![Token::IntegerLiteral(int)] }

#[test]
fn integer_literal_base_10() {
    assert_eq!(lex("0"), literal_u64(0));
    assert_eq!(lex("1"), literal_u64(1));
    assert_eq!(lex("2"), literal_u64(2));
    assert_eq!(lex("3"), literal_u64(3));
    assert_eq!(lex("4"), literal_u64(4));
    assert_eq!(lex("5"), literal_u64(5));
    assert_eq!(lex("6"), literal_u64(6));
    assert_eq!(lex("7"), literal_u64(7));
    assert_eq!(lex("8"), literal_u64(8));
    assert_eq!(lex("9"), literal_u64(9));
    assert_eq!(lex(&u64::MAX.to_string()), literal_u64(u64::MAX));
}
