use nom::IResult;

use {Kind, Position, Span, Token};
use interner::StringInterner;

macro_rules! spanned_regex {
    ($i:ident, $re:expr) => {
        ::lexer::unicode::restore_span($i, re_find_static!($i.fragment, $re))
    };
}

mod literal;
mod unicode;
mod whitespace;

/// Parse a token from the front of the span
pub fn token<'i, 'lex>(i: Span<'i>, pool: &'lex StringInterner)-> IResult<Span<'i>, Token<'lex>> {
    alt!(i,
        call!(whitespace::whitespace, pool) |
        call!(literal::integer, pool) |
        call!(literal::string, pool) |
        call!(symbol, pool) |
        call!(identifier, pool) |
        call!(unknown, pool)
    )
}

/// `Kind::Symbol`
fn symbol<'i, 'lex>(i: Span<'i>, pool: &'lex StringInterner)-> IResult<Span<'i>, Token<'lex>> {
    do_parse!(i,
        pos: position!() >>
        symbol: call!(unicode::symbol) >>
        (Token::new(
            Position(pos),
            pool.get_or_insert(symbol.fragment),
            Kind::Symbol,
        ))
    )
}

/// `Kind::Identifier`
fn identifier<'i, 'lex>(i: Span<'i>, pool: &'lex StringInterner) -> IResult<Span<'i>, Token<'lex>> {
    do_parse!(i,
        pos: position!() >>
        ident: call!(unicode::identifier) >>
        (Token::new(
            Position(pos),
            pool.get_or_insert(ident.fragment),
            Kind::Identifier,
        ))
    )
}

/// `Kind::Unknown`
fn unknown<'i, 'lex>(i: Span<'i>, pool: &'lex StringInterner) -> IResult<Span<'i>, Token<'lex>> {
    do_parse!(i,
        pos: position!() >>
        ch: take!(1) >>
        (Token::new(
            Position(pos),
            pool.get_or_insert(ch.fragment),
            Kind::Unknown,
        ))
    )
}
