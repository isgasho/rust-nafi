use nom::IResult;
use tokens::code::*;
use {Cursor, Span};

named! { pub line (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        comment: spanned_regex!("(?-s)^//.*") >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            comment.fragment.0,
            Kind::Comment(CommentStyle::Line),
        ))
    )
}

named! { pub line_doc (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        comment: spanned_regex!("(?-s)^///.*") >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            comment.fragment.0,
            Kind::Comment(CommentStyle::LineDoc),
        ))
    )
}

fn block_impl(i: Cursor) -> IResult<Cursor, Cursor> {
    use nom::{InputLength, Slice};
    tag!(i, "/*")?;

    let mut idx: usize = 2;
    let mut depth: u32 = 1;

    while depth > 0 && idx < i.input_len() {
        alt!(i.slice(idx..),
            tag!("/*")                => {|_:Cursor| { depth += 1; idx += 2; }} |
            tag!("*/")                => {|_:Cursor| { depth -= 1; idx += 2; }} |
            take_until_either1!("/*") => {|o:Cursor| { idx += o.input_len(); }} |
            take!(1)                  => {|o:Cursor| { idx += o.input_len(); }}
        )?;
    }

    Ok((i.slice(idx..), i.slice(..idx)))
}

named! { pub block (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        block: call!(block_impl) >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            block.fragment.0,
            Kind::Comment(CommentStyle::Block),
        ))
    )
}

named! { pub block_doc (Cursor) -> Token,
    do_parse!(
        start: position!() >>
        peek!(tag!("/**")) >>
        block: call!(block_impl) >>
        stop: position!() >>
        (Token::new(
            Span(start, stop),
            block.fragment.0,
            Kind::Comment(CommentStyle::BlockDoc),
        ))
    )
}

#[cfg(test)]
mod test {
    #![allow(unsafe_code)]

    use Cursor;
    use nom::Slice;

    #[test]
    fn block_a() {
        let comment = unsafe { Cursor("/* /* *** */ */ ", 0, 0) };
        assert_eq!(
            super::block_impl(comment),
            Ok((comment.slice(15..), comment.slice(..15)))
        );
    }

    #[test]
    fn block_b() {
        let comment = unsafe { Cursor("/* /* /** */ */ ", 0, 0) };
        assert_eq!(
            super::block_impl(comment),
            Ok((comment.slice(16..), comment.slice(..16)))
        );
    }

    #[test]
    fn block_c() {
        let comment = unsafe { Cursor("/* /* */* */ */ ", 0, 0) };
        assert_eq!(
            super::block_impl(comment),
            Ok((comment.slice(12..), comment.slice(..12)))
        );
    }

    #[test]
    fn block_d() {
        let comment = unsafe { Cursor("/* /* **/ */ */ ", 0, 0) };
        assert_eq!(
            super::block_impl(comment),
            Ok((comment.slice(12..), comment.slice(..12)))
        );
    }
}
