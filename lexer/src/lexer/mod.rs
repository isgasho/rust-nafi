use nom::IResult;
use tokens::{Kind, Token};
use interner::StringInterner;
use location::Span;

use {Position, Cursor};

macro_rules! spanned_regex {
    ($i:ident, $re:expr) => {
        #[cfg_attr(feature = "cargo-clippy", allow(regex_macro))] // false positive rust-clippy#2586
        ::lexer::unicode::restore_span($i, re_find_static!($i.fragment, $re))
    };
}

mod literal;
mod unicode;
mod whitespace;

/// Parse a token from the front of the cursor
pub(crate) fn token<'i, 'lex>(i: Cursor<'i>, pool: &'lex StringInterner)-> IResult<Cursor<'i>, Token<'lex>> {
    let (i,()) = whitespace::skip_whitespace(i)?;
    alt!(i,
        call!(literal::integer, pool) |
        call!(literal::string, pool) |
        call!(symbol, pool) |
        call!(identifier, pool) |
        call!(unknown, pool)
    )
}

/// `Kind::Symbol`
fn symbol<'i, 'lex>(i: Cursor<'i>, pool: &'lex StringInterner)-> IResult<Cursor<'i>, Token<'lex>> {
    do_parse!(i,
        start: position!() >>
        symbol: call!(unicode::symbol) >>
        stop: position!() >>
        (Token::new(
            Span { start:Position(start), stop:Position(stop) },
            pool.get_or_insert(symbol.fragment),
            Kind::Symbol,
        ))
    )
}

/// `Kind::Identifier`
fn identifier<'i, 'lex>(i: Cursor<'i>, pool: &'lex StringInterner) -> IResult<Cursor<'i>, Token<'lex>> {
    do_parse!(i,
        start: position!() >>
        ident: call!(unicode::identifier) >>
        stop: position!() >>
        (Token::new(
            Span { start:Position(start), stop:Position(stop) },
            pool.get_or_insert(ident.fragment),
            Kind::Identifier,
        ))
    )
}

/// `Kind::Unknown`
fn unknown<'i, 'lex>(i: Cursor<'i>, pool: &'lex StringInterner) -> IResult<Cursor<'i>, Token<'lex>> {
    do_parse!(i,
        start: position!() >>
        ch: take!(1) >>
        stop: position!() >>
        (Token::new(
            Span { start:Position(start), stop:Position(stop) },
            pool.get_or_insert(ch.fragment),
            Kind::Unknown,
        ))
    )
}
