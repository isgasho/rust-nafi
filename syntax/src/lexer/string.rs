use memchr::{memchr, memchr2};

Kind! {
    Text,
    EscapedCarriageReturn,
    EscapedNewLine,
    EscapedTab,
    EscapedBackslash,
    EscapedQuote,
    EscapedUnicode,
    InvalidEscape,
    InterpolationStart,
    StringEnd,
}

pub fn lex(source: &str) -> Option<Token> {
    None.or_else(|| unicode_escape(source))
        .or_else(|| simple_escape(source))
        .or_else(|| string_end(source))
        .or_else(|| text(source))
}

fn simple_escape(source: &str) -> Option<Token> {
    if !source.starts_with('\\') {
        return None;
    }
    let escape = match source.chars().nth(2) {
        Some(c) => c,
        None => {
            return Some(Token {
                source,
                kind: Kind::InvalidEscape,
            })
        },
    };
    let kind = match escape {
        'r' => Kind::EscapedCarriageReturn,
        'n' => Kind::EscapedNewLine,
        't' => Kind::EscapedTab,
        '\\' => Kind::EscapedBackslash,
        '"' => Kind::EscapedQuote,
        'u' if source[2..].starts_with('{') => return None, // handled by unicode_escape
        '{' => Kind::InterpolationStart,
        _ => Kind::InvalidEscape,
    };
    Some(Token {
        source: &source[..1 + escape.len_utf8()],
        kind,
    })
}

fn unicode_escape(source: &str) -> Option<Token> {
    if !source.starts_with("\\u{") {
        return None;
    }
    let end_idx = match memchr(b'}', source.as_bytes()) {
        Some(idx) => idx,
        None => {
            return Some(Token {
                source,
                kind: Kind::InvalidEscape,
            })
        },
    };
    let payload = &source[3..end_idx];
    Some(Token {
        source: &source[..=end_idx],
        kind: if payload.len() >= 4 && payload.len() <= 6
            && payload.bytes().all(|b| b.is_ascii_hexdigit())
        {
            Kind::EscapedUnicode
        } else {
            Kind::InvalidEscape
        },
    })
}

fn text(source: &str) -> Option<Token> {
    let idx = memchr2(b'"', b'\\', source.as_bytes()).unwrap_or_else(|| source.len());
    if idx > 0 {
        Some(Token {
            source: &source[..idx],
            kind: Kind::Text,
        })
    } else {
        None
    }
}

fn string_end(source: &str) -> Option<Token> {
    if source.starts_with('"') {
        Some(Token {
            source: &source[..1],
            kind: Kind::StringEnd,
        })
    } else {
        None
    }
}
