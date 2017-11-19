extern crate nnom;
use nnom::combinators::many0;
use nnom::prelude::*;

#[test]
fn many0_str() {
    fn tag(input: &str) -> ParseResult<&str, &str, ()> {
        if input.starts_with("Q") {
            let (output, remaining_input) = input.split_at(1);
            Ok(ParseOutput {
                remaining_input,
                output,
            })
        } else {
            Err(())
        }
    }

    assert_eq!(
        many0(tag)("QQrest"),
        Ok(ParseOutput {
            remaining_input: "rest",
            output: vec!["Q", "Q"],
        })
    )
}

#[test]
fn many0_slice() {
    fn tag(input: &[u32]) -> ParseResult<&[u32], &[u32], ()> {
        if input.starts_with(&[0]) {
            let (output, remaining_input) = input.split_at(1);
            Ok(ParseOutput {
                remaining_input,
                output,
            })
        } else {
            Err(())
        }
    }

    assert_eq!(
        many0(tag)(&[0, 0, 1]),
        Ok(ParseOutput::<&[u32], Vec<&[u32]>> {
            remaining_input: &[1],
            output: vec![&[0], &[0]],
        })
    )
}

#[test]
fn many0_positioned_str() {
    fn tag(input: PositionedStr) -> ParseResult<PositionedStr, PositionedStr, ()> {
        if input.starts_with("Q") {
            let (output, remaining_input) = input.split_at(1);
            Ok(ParseOutput {
                remaining_input,
                output,
            })
        } else {
            Err(())
        }
    }

    assert_eq!(
        many0(tag)(PositionedStr::from("QQRest")),
        Ok(ParseOutput {
            remaining_input: PositionedStr::new("Rest", 2),
            output: vec![PositionedStr::new("Q", 0), PositionedStr::new("Q", 1)],
        })
    )
}

#[test]
fn many0_positioned_slice() {
    fn tag(
        input: PositionedSlice<u32>,
    ) -> ParseResult<PositionedSlice<u32>, PositionedSlice<u32>, ()> {
        if input.starts_with(&[0]) {
            let (output, remaining_input) = input.split_at(1);
            Ok(ParseOutput {
                remaining_input,
                output,
            })
        } else {
            Err(())
        }
    }

    assert_eq!(
        many0(tag)(PositionedSlice::from([0, 0, 1].as_ref())),
        Ok(ParseOutput {
            remaining_input: PositionedSlice::new(&[1], 2),
            output: vec![PositionedSlice::new(&[0], 0), PositionedSlice::new(&[0], 1)],
        })
    )
}
