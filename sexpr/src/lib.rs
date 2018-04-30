#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;
extern crate optional;

use optional::{none, some, Optioned};
use std::fmt;
use std::str::FromStr;
pub use syn::synom::ParseError;
use syn::synom::Synom;
use syn::token::Paren;
use syn::Ident;
pub use syn::Lit;

#[derive(Copy, Clone, Debug)]
pub enum SExprHead {
    Symbol(Ident),
    Omit(Token![_]),
}

impl Synom for SExprHead {
    named!(parse -> Self, alt!(
        syn!(Ident) => {SExprHead::Symbol}
        |
        syn!(Token![_]) => {SExprHead::Omit}
    ));
}

impl fmt::Display for SExprHead {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExprHead::Symbol(ident) => write!(f, "{}", ident.as_ref()),
            SExprHead::Omit(line) => write!(f, "{}", quote!(#line)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SExprTail {
    Symbol(Ident),
    Literal(Lit),
}

impl Synom for SExprTail {
    named!(parse -> Self, alt!(
        syn!(Ident) => {SExprTail::Symbol}
        |
        syn!(Lit) => {SExprTail::Literal}
    ));
}

impl fmt::Display for SExprTail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExprTail::Symbol(ident) => write!(f, "{}", ident.as_ref()),
            SExprTail::Literal(lit) => write!(f, "{}", quote!(#lit)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SExpr {
    Pair(Paren, SExprHead, SExprTail),
    List(Paren, SExprHead, Vec<SExpr>),
    Tail(SExprTail),
}

impl Synom for SExpr {
    named!(parse -> Self, alt!(
        parens!(tuple!(syn!(SExprHead), syn!(SExprTail)))
            => {|(paren, (head, tail))| SExpr::Pair(paren, head, tail)}
        |
        parens!(tuple!(syn!(SExprHead), many0!(syn!(SExpr))))
            => {|(paren, (head, tail))| SExpr::List(paren, head, tail)}
        |
        syn!(SExprTail) => {SExpr::Tail}
    ));
}

impl FromStr for SExpr {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> { Ok(syn::parse_str(s)?) }
}

impl SExpr {
    pub fn single_line(&self) -> Display { self.display(none()) }

    pub fn multi_line(&self) -> Display { self.display(some(1)) }

    fn display(&self, depth: Optioned<usize>) -> Display { Display { depth, sexpr: self } }
}

#[derive(Copy, Clone, Debug)]
pub struct Display<'s> {
    sexpr: &'s SExpr,
    depth: Optioned<usize>,
}

impl<'s> Display<'s> {
    fn write_indent<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        if self.depth.is_some() {
            writeln!(w)?;
            for _ in 0..self.depth.unpack() {
                write!(w, " ")?;
            }
        } else {
            write!(w, " ")?;
        }
        Ok(())
    }
}

impl<'s> fmt::Display for Display<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sexpr {
            SExpr::Pair(_paren, head, tail) => write!(f, "({} {})", head, tail),
            SExpr::List(_paren, head, tail) => {
                write!(f, "({}", head)?;
                for child in tail {
                    self.write_indent(f)?;
                    write!(f, "{}", child.display(self.depth.map_t(|t| t + 1)))?;
                }
                write!(f, ")")
            },
            SExpr::Tail(tail) => write!(f, "{}", tail),
        }
    }
}
