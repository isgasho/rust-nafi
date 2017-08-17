use tokens::Token;
use nom::{not_line_ending, line_ending, space};

mod literals;
use self::literals::integer_literal;

/// Vec<Token>
named! {
    pub tokens<&str, Vec<Token>>,
    many0!(token)
}

/// Token
named! {
    token<&str, Token>,
    alt_complete!(
        integer_literal |
        _whitespace |
        _unknown
    )
}

/// Token::_Unknown
named! {
    _unknown<&str, Token>,
    do_parse!(
        take!(1) >>
        (Token::_Unknown)
    )
}

/// Token::_Whitespace
named! {
    _whitespace<&str, Token>,
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
        content: not_line_ending >>
        ()
    )
}

/// Consume a block comment
named! {
    take_block_comment<&str, ()>,
    do_parse!(
        tag!("/*") >>
        many0!(
            alt_complete!(
                take_block_comment |
                do_parse!(
                    not!(tag!("*/")) >>
                    ()
                )
            )
        ) >>
        alt_complete!(tag!("*/") | eof!()) >>
        ()
    )
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
    extern crate nom;
    use lexer;
    #[test]
    fn block_comment() {
        assert_eq!(lexer::take_block_comment("/*/**/*/"), nom::IResult::Done("", ()));
    }
}
