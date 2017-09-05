//! Test literals in NAFI source code, e.g. numbers, strings

extern crate nafi_lexer;
extern crate nafi_tokens;

use nafi_lexer::{lex, Token};
use nafi_tokens::BigUint;

fn literal_num<N: Into<BigUint>>(int: N) -> Vec<Token> {
    vec![Token::Literal(0, int.into().into())]
}

#[test]
fn integer_literal_base_10() {
    assert_eq!(lex("0"), literal_num(0u32));
    assert_eq!(lex("1"), literal_num(1u32));
    assert_eq!(lex("2"), literal_num(2u32));
    assert_eq!(lex("3"), literal_num(3u32));
    assert_eq!(lex("4"), literal_num(4u32));
    assert_eq!(lex("5"), literal_num(5u32));
    assert_eq!(lex("6"), literal_num(6u32));
    assert_eq!(lex("7"), literal_num(7u32));
    assert_eq!(lex("8"), literal_num(8u32));
    assert_eq!(lex("9"), literal_num(9u32));
}

#[test]
fn integer_literal_allows_very_big_numbers() {
    let bignum = "1234567890123456789012345678901234567890"; // roughly 2 ^ 129.859
    assert_eq!(
        lex(bignum),
        vec![Token::Literal(0, bignum.parse::<BigUint>().unwrap().into())]
    )
}
