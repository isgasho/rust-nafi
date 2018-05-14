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
extern crate num_bigint as bigint;
extern crate regex;
#[macro_use]
extern crate failure;
extern crate nafi_misc;

#[cfg(test)]
extern crate ron;

mod untyped;

mod lexer;
mod parser;
