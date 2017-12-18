#![allow(unused)]
use Span;
use lexer::unicode::white_space;
use nom::{IResult, InputLength, Slice};
use regex::Regex;
use tokens::Token;

/// `Token::Whitespace`
pub fn whitespace(i: Span) -> IResult<Span, Token> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    do_parse!(i,
        pos: position!() >>
        fold_many1!(
            alt_complete!(white_space | line_comment | block_comment),
            (), |_, _| ()
        ) >>
        (Token::Whitespace(pos.offset))
    )
}

/// Parse a line comment
fn line_comment(i: Span) -> IResult<Span, Span> {
    tag!(i, "//")?;

    lazy_static! {
        static ref NEWLINE: Regex = Regex::new(r"\n").unwrap();
    }

    let idx = i.fragment.find(&*NEWLINE).unwrap_or_else(|| i.input_len());
    Ok((i.slice(idx..), i.slice(..idx)))
}

/// Parse a block comment
fn block_comment(i: Span) -> IResult<Span, Span> {
    tag!(i, "/*")?;

    let mut idx: usize = 2;
    let mut depth = 1;

    while depth > 0 && idx < i.input_len() {
        let i = i.slice(idx..);
        #[cfg_attr(rustfmt, rustfmt_skip)]
        alt_complete!(i,
            tag!("/*")               => {|_:Span| { depth += 1; idx += 2; }} |
            tag!("*/")               => {|_:Span| { depth -= 1; idx += 2; }} |
            take_until_either!("/*") => {|o:Span| { idx += o.input_len(); }} |
            take_s!(1)               => {|o:Span| { idx += o.input_len(); }}
        );
    }

    Ok((i.slice(idx..), i.slice(..idx)))
}

#[cfg(test)]
mod test {
    use Span;
    use nom::Slice;
    #[test]
    fn block_comment() {
        let comment = Span::new("/** /* */*/");
        assert_eq!(
            super::block_comment(comment),
            Ok((comment.slice(11..), comment.slice(..11)))
        );
    }

    #[test]
    fn line_comment() {
        let comment = Span::new("// any amount of text you want");
        assert_eq!(
            super::line_comment(comment),
            Ok((comment.slice(30..), comment.slice(..30)))
        );
    }
}
