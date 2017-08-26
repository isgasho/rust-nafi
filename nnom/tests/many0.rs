#![feature(never_type)]
extern crate nnom;

use nnom::prelude::*;

#[test]
fn many0_str() {
    fn tag(input: &str) -> Result<&str, &str, ()> {
        if input.starts_with("Q") {
            Ok(input.split_at(1))
        } else {
            Err(())
        }
    }

    assert_eq!(many0(tag)("QQRest"), Ok((vec!["Q", "Q"], "Rest")))
}

#[test]
fn many0_slice() {
    fn tag(input: &[u32]) -> Result<&[u32], &[u32], ()> {
        if input.starts_with(&[0]) {
            Ok(input.split_at(1))
        } else {
            Err(())
        }
    }

    assert_eq!(
        many0(tag)(&[0, 0, 1]),
        // explicitly type Ok to coerce arrays to slices
        Ok::<(Vec<&[u32]>, &[u32]), !>((vec![&[0], &[0]], &[1]))
    )
}

#[test]
fn many0_positioned_str() {
    fn tag(input: PositionedStr) -> Result<PositionedStr, PositionedStr, ()> {
        if input.starts_with("Q") {
            Ok(input.split_at(1))
        } else {
            Err(())
        }
    }

    assert_eq!(
        many0(tag)(PositionedStr::from("QQRest")),
        Ok((
            vec![PositionedStr::new("Q", 0), PositionedStr::new("Q", 1)],
            PositionedStr::new("Rest", 2),
        ),)
    )
}
