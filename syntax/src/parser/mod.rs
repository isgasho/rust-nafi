//! Parser that turns nafi source into

use std::u32;
use optional::{Optioned, some, none};
use slog::Logger;
use nafi_misc::ReplaceMany;

use untyped::{SyntaxTree, Kind as SyntaxKind, Node as SyntaxNode};
use lexer::{self, code, string};

#[derive(Clone, Debug)]
struct ParseContext<'a> {
    pos: u32,
    parent: Optioned<u32>,
    previous: Optioned<u32>,
    source: &'a str,
    log: Logger,
}

#[derive(Copy, Clone, Debug, Fail)]
pub enum ParseFailure {
    #[fail(display = "Nafi doesn't support files over 4 GiB")]
    TooBig,
    #[fail(display = "The nafi parser hit the end of the file unexpectedly; this is a bug")]
    UnexpectedEof,
    #[doc(hidden)]
    #[fail(display = "OOPSIE WOOPSIE!! Uwu We made a fucky wucky!! A wittle fucko boingo! \
                      The code monkeys at our headquarters are working VEWY HAWD to fix this!")]
    __NonExhausive, // sorry, this should never be constructed
}

type Pos = Result<u32, ParseFailure>;

macro_rules! set {
    ($ctx:ident.$target:ident.$member:ident in $nodes:ident = $val:expr) => {
        if let Some(n) = $nodes.get_mut($ctx.$target.unpack() as usize) {
            n.$member = some(n.$member.unwrap_or($val));
        }
    };
}

fn push(ctx: &mut ParseContext, nodes: &mut Vec<SyntaxNode>, kind: SyntaxKind, length: u32) -> u32 {
    trace!(ctx.log, "{}", kind.as_str(); "length" => length);
    let pos = ctx.pos + length;
    let idx = nodes.len() as u32;
    nodes.push(SyntaxNode {
        kind,
        span: (ctx.pos, pos),
        parent: ctx.parent,
        previous: ctx.previous,
        child: none(),
        next: none(),
    });
    set!(ctx.parent.child in nodes = idx);
    set!(ctx.previous.next in nodes = idx);
    ctx.previous = some(idx);
    ctx.pos = pos;
    ctx.source = &ctx.source[length as usize..];
    pos
}

fn next(ctx: &mut ParseContext, nodes: &mut Vec<SyntaxNode>) -> Option<code::Token> {
    loop {
        let tok = code::lex(ctx.source)?;
        if tok.kind == code::Kind::Whitespace {
            push(ctx, nodes, SyntaxKind::Whitespace, tok.length);
        } else {
            return Some(tok);
        }
    }
}

fn source<'a>(ctx: &ParseContext<'a>, tok: code::Token) -> &'a str {
    &ctx.source[..tok.length as usize]
}

fn error_until(mut ctx: ParseContext, nodes: &mut Vec<SyntaxNode>, terminators: &[code::Kind]) -> u32 {
    ctx.log = ctx.log.new(o!("rule" => "error-until"));
    let start_idx = nodes.len() as u32;
    let start_pos = ctx.pos;
    let previous = ctx.previous;
    let parent = ctx.parent;
    ctx.previous = none();
    ctx.parent = none();
    loop {
        match next(&mut ctx, nodes) {
            Some(tok) if terminators.contains(&tok.kind) => match tok.kind {
                code::Kind::Identifier => push(&mut ctx, nodes, SyntaxKind::Identifier, tok.length),
                _ => unimplemented!(),
            },
            _ => break,
        };
    }
    let idx = nodes.len() as u32;
    for node in &mut nodes[start_idx as usize..] {
        node.parent = some(idx);
    }
    nodes.push(SyntaxNode {
        kind: SyntaxKind::ERROR,
        span: (start_pos, ctx.pos),
        parent, previous,
        child: if start_idx != idx { some(start_idx) } else { none() },
        next: none(),
    });
    ctx.pos
}

mod expression;

pub fn parse(source: String, log: &Logger) -> Result<SyntaxTree, ParseFailure> {
    let log = log.new(o!(
        "source" => source.replace_many(&[("\\", r#"\\"#), ("\n", r#"\n"#)]),
    ));
    if source.len() > u32::MAX as usize {
        Err(ParseFailure::TooBig)?;
    }
    let mut nodes = vec![];
    let consumed = expression::maybe_binary(
        ParseContext {
            pos: 0,
            parent: none(),
            previous: none(),
            source: &source,
            log,
        },
        &mut nodes
    );
    if consumed < source.len() as u32 {
        unimplemented!("Excess input");
    }
    Ok(SyntaxTree { source, nodes })
}

#[cfg(test)]
mod tests {
    extern crate slog_term;
    extern crate ron;
    use super::*;
    use nafi_misc::PrintWriter;

    #[inline(always)]
    fn make_logger() -> ::slog::Logger {
        use slog::{Drain, Logger};
        use std::sync::Mutex;
        Logger::root(
            Mutex::new(
                slog_term::CompactFormat::new(
                    slog_term::PlainSyncDecorator::new(PrintWriter)
                ).build().fuse()
            ).fuse(),
            o!(),
        )
    }
}
