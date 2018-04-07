use nom::{IResult, Slice, InputLength};

use {Cursor, lexer::unicode::white_space};

pub(crate) fn skip_whitespace(i: Cursor) -> IResult<Cursor, ()> {
    fold_many0!(i,
        alt!(white_space | line_comment | block_comment),
        (), |_, _| ()
    )
}

/// Parse a line comment
fn line_comment(i: Cursor) -> IResult<Cursor, Cursor> {
    spanned_regex!(i, "^//.*?(?m:$)")
}

/// Parse a block comment
fn block_comment(i: Cursor) -> IResult<Cursor, Cursor> {
    tag!(i, "/*")?;

    let mut idx: usize = 2;
    let mut depth = 1;

    while depth > 0 && idx < i.input_len() {
        // FIXME(Geal/nom#696): Unused `use` in macro
        #[allow(unused)]
        alt!(i.slice(idx..),
            tag!("/*")                => {|_:Cursor| { depth += 1; idx += 2; }} |
            tag!("*/")                => {|_:Cursor| { depth -= 1; idx += 2; }} |
            take_until_either1!("/*") => {|o:Cursor| { idx += o.input_len(); }} |
            take!(1)                  => {|o:Cursor| { idx += o.input_len(); }}
        )?;
    }

    Ok((i.slice(idx..), i.slice(..idx)))
}

#[cfg(test)]
mod test {
    use Cursor;
    use nom::Slice;
    #[test]
    fn block_comment() {
        let comment = Cursor::new("/* /* *** */ */");
        assert_eq!(
            super::block_comment(comment),
            Ok((comment.slice(15..), comment.slice(..15)))
        );
    }

    #[test]
    fn line_comment() {
        let comment = Cursor::new("// any amount of text you want");
        assert_eq!(
            super::line_comment(comment),
            Ok((comment.slice(30..), comment.slice(..30)))
        );
    }
}
