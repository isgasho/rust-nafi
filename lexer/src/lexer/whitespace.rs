use nom::{IResult, Slice, InputLength};

use {Kind, Position, Span, Token};
use interner::StringInterner;
use lexer::unicode::white_space;

/// `Kind::Whitespace`
pub fn whitespace<'i, 'lex>(i: Span<'i>, pool: &'lex StringInterner) -> IResult<Span<'i>, Token<'lex>> {
    do_parse!(i,
        pos: position!() >>
        s: fold_many1!(
            alt!(white_space | line_comment | block_comment),
            pos,
            |prefix: Span<'i>, suffix: Span<'i>| i.slice(..prefix.input_len() + suffix.input_len())
        ) >>
        (Token::new(
            Position(pos),
            pool.get_or_insert(s.fragment),
            Kind::Whitespace
        ))
    )
}

/// Parse a line comment
fn line_comment(i: Span) -> IResult<Span, Span> {
    spanned_regex!(i, "^//.*?(?m:$)")
}

/// Parse a block comment
fn block_comment(i: Span) -> IResult<Span, Span> {
    tag!(i, "/*")?;

    let mut idx: usize = 2;
    let mut depth = 1;

    while depth > 0 && idx < i.input_len() {
        alt!(i.slice(idx..),
            tag!("/*")                => {|_:Span| { depth += 1; idx += 2; }} |
            tag!("*/")                => {|_:Span| { depth -= 1; idx += 2; }} |
            // FIXME(Geal/nom#680): Should be `take_until_either1!`
            take_until_either!("/*")  => {|o:Span| { idx += o.input_len(); }} |
            take!(1)                  => {|o:Span| { idx += o.input_len(); }}
        )?;
    }

    Ok((i.slice(idx..), i.slice(..idx)))
}

#[cfg(test)]
mod test {
    use Span;
    use nom::Slice;
    #[test]
    fn block_comment() {
        let comment = Span::new("/* /* *** */ */");
        assert_eq!(
            super::block_comment(comment),
            Ok((comment.slice(15..), comment.slice(..15)))
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
