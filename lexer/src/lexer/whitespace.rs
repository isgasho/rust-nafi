use tokens::Token;
use nom::{line_ending, not_line_ending, space, IResult};

/// Token::_Whitespace
named! {
    pub _whitespace<&str, Token>,
    do_parse!(
        many1!(
            alt_complete!(
                take_comment |
                take_whitespace
            )
        ) >>
        (Token::_Whitespace)
    )
}

/// Consume a comment
named! {
    take_comment<&str, ()>,
    do_parse!(
        alt_complete!(
            take_line_comment |
            take_block_comment
        ) >>
        ()
    )
}

/// Consume a line comment
named! {
    take_line_comment<&str, ()>,
    do_parse!(
        tag!("//") >>
        not_line_ending >>
        ()
    )
}

/// Consume a block comment
fn take_block_comment(i: &str) -> IResult<&str, ()> {
    match tag!(i, "/*") {
        IResult::Done(mut i, _) => {
            let mut depth = 1;
            while depth > 0 {
                match take!(i, 2) {
                    IResult::Done(_i, o) if o == "/*" => {
                        depth += 1;
                        i = _i;
                    },
                    IResult::Done(_i, o) if o == "*/" => {
                        depth -= 1;
                        i = _i;
                    },
                    IResult::Done(..) => {
                        i = take!(i, 1).unwrap().0;
                    },
                    IResult::Incomplete(_) => {
                        assert!(
                            IResult::<&str, &str, u32>::is_done(&eof!(i,),) ||
                                IResult::<&str, &str, u32>::is_done(&eof!(take!(i, 1).unwrap().0,),)
                        );
                        break;
                    },
                    e @ IResult::Error(_) => return e.map(|_| ()),
                }
            }
            IResult::Done(i, ())
        },
        result => result.map(|_| ()),
    }
}

/// Consume as much whitespace as possible
named! {
    take_whitespace<&str, ()>,
    do_parse!(
        many1!(
            alt_complete!(
                // TODO: Pattern_White_Space
                space | line_ending
            )
        ) >>
        ()
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn block_comment() {
        assert_eq!(
            take_block_comment("/** /* */*/"),
            IResult::Done("", ())
        );
    }

    #[test]
    fn line_comment() {
        assert_eq!(
            take_line_comment("// any amount of stupid text you want"),
            IResult::Done("", ())
        );
    }
}
