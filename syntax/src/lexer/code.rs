use std::u32;
use memchr::memchr2;

Kind! {
    /// A name that refers to some binding, meeting UAX31-R1 unmodified
    Identifier,
    /// A non-ASCII symbol or punctuation
    // TODO(CAD97): Separate token for emoji (handling emoji joining)?
    Symbol,
    /// A decimal integer of digits 0-9 optionally with medial underscores and `0d` prefix
    DecimalInteger,
    /// A continuous region of whitespace matching Unicode White_Space
    Whitespace,
    /// A comment starting with `//` and ending with a newline
    LineComment,
    /// A comment starting with `///` and ending with a newline
    LineDocComment,
    /// A comment starting with `/*` and ending with `*/`
    BlockComment,
    /// A comment starting with `/**` and ending with `*/`
    BlockDocComment,
    /// A character not matched by one of the above rules
    Invalid,
}

pub fn lex(source: &str) -> Option<Token> {
    None.or_else(|| identifier(source))
        .or_else(|| symbol(source))
        .or_else(|| decimal_integer(source))
        .or_else(|| whitespace(source))
        .or_else(|| line_comment(source))
        .or_else(|| block_comment(source))
        .or_else(|| invalid(source))
}

fn identifier(source: &str) -> Option<Token> {
    re!(r#"^[\p{XID_START}][\p{XID_CONTINUE}]*"#)
        .find(source)
        .map(|m| Token {
            length: m.end() as u32,
            kind: Kind::Identifier,
        })
}

fn symbol(source: &str) -> Option<Token> {
    re!(r#"^[\p{Punctuation}\p{Symbol}]"#)
        .find(source)
        .map(|m| Token {
            length: m.end() as u32,
            kind: Kind::Symbol,
        })
}

fn decimal_integer(source: &str) -> Option<Token> {
    re!(r#"^(?:0d_?)?[[:digit:]][[:digit:]_]*"#)
        .find(source)
        .map(|m| Token {
            length: m.end() as u32,
            kind: Kind::DecimalInteger,
        })
}

fn whitespace(source: &str) -> Option<Token> {
    re!(r#"^[\p{White_Space}]+"#).find(source).map(|m| Token {
        length: m.end() as u32,
        kind: Kind::Whitespace,
    })
}

fn line_comment(source: &str) -> Option<Token> {
    re!(r#"^//.*"#).find(source).map(|m| Token {
        length: m.end() as u32,
        kind: if source[2..].starts_with('/') {
            Kind::LineDocComment
        } else {
            Kind::LineComment
        },
    })
}

fn block_comment(source: &str) -> Option<Token> {
    if !source.starts_with("/*") {
        return None;
    }

    let mut idx: usize = 2;
    let mut depth: u32 = 1;

    while depth > 0 && idx < source.len() {
        let source = &source[idx..];
        if source.starts_with("/*") {
            depth += 1;
            idx += 2;
        } else if source.starts_with("*/") {
            depth -= 1;
            idx += 2;
        } else {
            let interesting_idx =
                memchr2(b'*', b'/', source.as_bytes()).unwrap_or_else(|| source.len());
            if interesting_idx > 0 {
                idx += interesting_idx;
            } else {
                idx += 1;
            }
        }
    }

    Some(Token {
        length: idx as u32,
        kind: if source[2..].starts_with('*') && !source[3..].starts_with('/') {
            Kind::BlockDocComment
        } else {
            Kind::BlockComment
        },
    })
}

fn invalid(source: &str) -> Option<Token> {
    re!(r#"^."#).find(source).map(|m| Token {
        length: m.end() as u32,
        kind: Kind::Invalid,
    })
}
