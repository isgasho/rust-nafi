extern crate nnom;

use nnom::prelude::*;

#[test]
fn many0_str() {
    fn tag(input: &str) -> Result<&str> {
        if input.starts_with("Q") {
            Result::Done(&input[1..], &input[..1])
        } else {
            Result::Pass
        }
    }

    assert_eq!(many0(tag)("QQRest"), Result::Done("Rest", vec!["Q", "Q"]))
}

#[test]
fn many0_slice() {
    fn tag(input: &[u32]) -> Result<&[u32]> {
        if input.starts_with(&[0]) {
            Result::Done(&input[1..], &input[..1])
        } else {
            Result::Pass
        }
    }

    assert_eq!(
        many0(tag)(&[0, 0, 1]),
        Result::Done::<&[u32], Vec<&[u32]>>(&[1], vec![&[0], &[0]])
    )
}

#[test]
fn many0_positioned_str() {
    fn tag(input: PositionedStr) -> Result<PositionedStr> {
        if input.starts_with("Q") {
            let split = input.split_at(1);
            Result::Done(split.1, split.0)
        } else {
            Result::Pass
        }
    }

    assert_eq!(
        many0(tag)(PositionedStr::from("QQRest")),
        Result::Done(
            PositionedStr::new("Rest", 2),
            vec![PositionedStr::new("Q", 0), PositionedStr::new("Q", 1)]
        )
    )
}
