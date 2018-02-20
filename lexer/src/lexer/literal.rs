use nom::{IResult, Slice, InputLength};
use nom::digit1;

use {Kind, Position, Span, Token};
use interner::StringInterner;
use lexer::token;
use tokens::{StringFragment, StringFragments};

/// `Kind::LiteralString`
pub fn string<'i, 'lex>(i: Span<'i>, pool: &'lex StringInterner)-> IResult<Span<'i>, Token<'lex>> {
    let (mut rest, pos) = tag!(i, "\"")?;
    let mut fragments = StringFragments::default();

    {
        let mut parse_string = || {
            loop {
                let (i, o) = take!(rest, 1)?;
                match o.fragment {
                    "\"" => {
                        rest = i;
                        break;
                    },
                    "\\" => {
                        rest = i;
                        let (ii, o) = take!(rest, 1)?;
                        rest = ii;
                        match o.fragment {
                            "r" => fragments.push(StringFragment::Escaped('\r')),
                            "n" => fragments.push(StringFragment::Escaped('\n')),
                            "t" => fragments.push(StringFragment::Escaped('\t')),
                            "\\" => fragments.push(StringFragment::Escaped('\\')),
                            // TODO: \u
                            "{" => {
                                let mut tokens = vec![];
                                let mut depth = 1_u32;
                                {
                                    let mut parse_interpolated = || {
                                        loop {
                                            let (i, o) = token(rest, pool)?;
                                            rest = i;
                                            match o {
                                                Token { kind: Kind::Symbol, source: "}", .. } => {
                                                    depth -= 1;
                                                    if depth == 0 {
                                                        break;
                                                    }
                                                },
                                                Token { kind: Kind::Symbol, source: "{", .. } => depth += 1,
                                                _ => {},
                                            }
                                            tokens.push(o);
                                        }
                                        Ok((rest, ()))
                                    };
                                    let res: IResult<Span<'i>, ()> = parse_interpolated();
                                    if res.is_err() {
                                        warn!("Unterminated String Interpolation");
                                    }
                                }

                                fragments.push(StringFragment::Interpolated(tokens));
                            }
                            _ => fragments.push(StringFragment::InvalidEscape(
                                Position(i),
                                pool.get_or_insert(i.slice(..i.input_len() - ii.input_len()).fragment),
                            )),
                        }
                    },
                    _ => {
                        // FIXME(Geal/nom#696): Unused `use` in macro
                        #[allow(unused)]
                        let (i, o) = match take_until_either1!(rest, "\\\"") {
                            Ok((i, o)) => (i, o),
                            Err(_) => (rest.slice(rest.input_len()..), rest),
                        };
                        rest = i;
                        fragments.push(pool.get_or_insert(o.fragment).into())
                    }
                }
            }
            Ok((rest, ()))
        };
        let res: IResult<Span<'i>, ()> = parse_string();
        if res.is_err() {
            warn!("Unterminated String");
        }
    }

    Ok((rest, Token::new(
        Position(pos),
        pool.get_or_insert(i.slice(..i.input_len()-rest.input_len()).fragment),
        Kind::LiteralString(fragments),
    )))
}

/// `Kind::LiteralInteger`
pub fn integer<'i, 'lex>(i: Span<'i>, pool: &'lex StringInterner)-> IResult<Span<'i>, Token<'lex>> {
    do_parse!(i,
        pos: position!() >>
        o: call!(digit1) >>
        (Token::new(
            Position(pos),
            pool.get_or_insert(o.fragment),
            Kind::LiteralInteger,
        ))
    )
}
