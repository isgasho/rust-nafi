use nom::IResult;
use Span;
use tokens::BigUint;
use nom::{InputLength, Slice, alpha, digit};

pub fn is_newline(ch: char) -> bool { matches!(ch as u32, 0xA..=0xD | 0x85 | 0x2028 | 0x2029) }
pub fn is_digit(ch: char) -> bool { matches!(ch as u32, 0x30..=0x39) }

pub fn white_space(i: Span) -> IResult<Span, Span> {
    take_while1!(i, char::is_whitespace)
}

pub fn identifier(i: Span) -> IResult<Span, Span> {
    // TODO: Use Unicode UAX31-R1 instead of this simple definition
    do_parse!(i,
        peek!(alpha) >>
        o: take_while!(char::is_alphanumeric) >>
        (o)
    )
}

pub fn decimal_number(i: Span) -> IResult<Span, BigUint> {
    digit(i).map(|(i, o)| (i, o.fragment.parse().unwrap()))
}
