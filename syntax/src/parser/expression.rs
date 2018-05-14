use super::{next, push, source, ParseContext, SyntaxKind, SyntaxNode};
use lexer::code::{Kind, Token};

pub(super) fn maybe_binary(mut ctx: ParseContext, nodes: &mut Vec<SyntaxNode>) -> u32 {
    unimplemented!()
}

pub(super) fn maybe_suffix(mut ctx: ParseContext, nodes: &mut Vec<SyntaxNode>) -> u32 {
    unimplemented!()
}

pub(super) fn simple(mut ctx: ParseContext, nodes: &mut Vec<SyntaxNode>) -> u32 {
    ctx.log = ctx.log.new(o!("rule" => "expression/simple"));
    match next(&mut ctx, nodes) {
        Some(tok) => match tok.kind {
            Kind::Identifier => identifier(ctx, nodes, tok),
            Kind::Symbol if source(&ctx, tok) == "(" => parenthesized(ctx, nodes, tok),
            _ => unimplemented!(),
        },
        None => push(&mut ctx, nodes, SyntaxKind::ERROR, 0),
    }
}

pub(super) fn identifier(mut ctx: ParseContext, nodes: &mut Vec<SyntaxNode>, head: Token) -> u32 {
    ctx.log = ctx.log.new(o!("rule" => "expression/identifier"));
    match next(&mut ctx, nodes) {
        Some(tok) if tok.kind == Kind::Symbol && source(&ctx, tok) == "(" => {
            function_call(ctx, nodes, head, tok)
        },
        _ => push(&mut ctx, nodes, SyntaxKind::Identifier, head.length),
    }
}

pub(super) fn parenthesized(
    mut ctx: ParseContext,
    nodes: &mut Vec<SyntaxNode>,
    open: Token,
) -> u32 {
    unimplemented!()
}

pub(super) fn function_call(
    mut ctx: ParseContext,
    nodes: &mut Vec<SyntaxNode>,
    name: Token,
    open: Token,
) -> u32 {
    ctx.log = ctx.log.new(o!("rule" => "expression/function-call"));
    unimplemented!()
}
