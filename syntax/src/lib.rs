#![allow(unused)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
extern crate bytecount;
extern crate memchr;
extern crate optional;
#[macro_use]
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
extern crate ron;
#[cfg(test)]
extern crate sexpr;

mod untyped;

mod lexer;
