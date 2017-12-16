use nom::IResult;
use single::Single;

use Span;
use tokens::{Keyword, Symbol, Token};

//mod literals;
mod unicode;
mod regex;
//mod whitespace;

//use self::literals::{integer_literal, string_literal};
use lexer::regex::restore_span;
//use self::whitespace::whitespace;

pub fn tokens(i: Span) -> IResult<Span, Vec<Token>> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    complete!(i, many0!(token))
}

fn token(i: Span) -> IResult<Span, Token> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    alt_complete!(i,
        symbol |
        identifier_like |
        _unknown
    )
}

/// `Token::Symbol`
fn symbol(i: Span) -> IResult<Span, Token> {
    let pos = i.offset;
    unicode::symbol(i).map(|(i, o)| {
        let ch = o.fragment.chars().single().unwrap();
        (i, Token::Symbol(pos, ch.into()))
    })
}

/// `Token::Identifier` or `Token::Keyword`
fn identifier_like(i: Span) -> IResult<Span, Token> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    do_parse!(i,
        pos: position!() >>
        o: call!(unicode::identifier) >>
        (match o.fragment {
            // TODO: Keyword map
            "let" => Token::Keyword(pos.offset, Keyword::Let),
            "mutable" => Token::Keyword(pos.offset, Keyword::Mutable),
            "if" => Token::Keyword(pos.offset, Keyword::If),
            "else" => Token::Keyword(pos.offset, Keyword::Else),
            ident => Token::Identifier(pos.offset, ident.into()),
        })
    )
}

/// `Token::_Unknown`
fn _unknown(i: Span) -> IResult<Span, Token> {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    do_parse!(i,
        pos: position!() >>
        ch: take_s!(1) >>
        (Token::_Unknown(pos.offset, ch.fragment.chars().single().unwrap()))
    )
}

/*
fn token(input: PositionedStr) -> ParseResult<PositionedStr, Token, ()> {
    Err(())
        .or_else(|_| whitespace(input))
        .or_else(|_| integer_literal(input))
        .or_else(|_| string_literal(input))
        .or_else(|_| identifier_like(input))
        .or_else(|_| symbol(input))
        .or_else(|_| _unknown(input))
}
*/
