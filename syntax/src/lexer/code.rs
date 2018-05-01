use memchr::memchr2;

Kind! {
    Identifier,
    Symbol,
    DecimalInteger,
    StringStart,
    Whitespace,
    LineComment,
    LineDocComment,
    BlockComment,
    BlockDocComment,
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
            source: m.as_str(),
            kind: Kind::Identifier,
        })
}

fn symbol(source: &str) -> Option<Token> {
    re!(r#"^[\p{Punctuation}\p{Symbol}]"#)
        .find(source)
        .map(|m| Token {
            source: m.as_str(),
            kind: if m.as_str() == "\"" {
                Kind::StringStart
            } else {
                Kind::Symbol
            },
        })
}

fn decimal_integer(source: &str) -> Option<Token> {
    re!(r#"^(?:0d_?)?[[:digit:]][[:digit:]_]*"#)
        .find(source)
        .map(|m| Token {
            source: m.as_str(),
            kind: Kind::DecimalInteger,
        })
}

fn whitespace(source: &str) -> Option<Token> {
    re!(r#"^[\p{White_Space}]+"#).find(source).map(|m| Token {
        source: m.as_str(),
        kind: Kind::Whitespace,
    })
}

fn line_comment(source: &str) -> Option<Token> {
    re!(r#"^//.*"#).find(source).map(|m| Token {
        source: m.as_str(),
        kind: if m.as_str()[2..].starts_with('/') {
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
        source: &source[..idx],
        kind: if source[2..].starts_with('*') && !source[3..].starts_with('/') {
            Kind::BlockDocComment
        } else {
            Kind::BlockComment
        },
    })
}

fn invalid(source: &str) -> Option<Token> {
    re!(r#"^."#).find(source).map(|m| Token {
        source: m.as_str(),
        kind: Kind::Invalid,
    })
}
