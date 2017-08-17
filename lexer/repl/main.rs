extern crate nafi_lexer;

use std::io;

fn main() {
    let mut buffer = String::with_capacity(80);
    loop {
        buffer.clear();
        if io::stdin().read_line(&mut buffer).is_err() {
            break
        }
        if buffer.is_empty() {
            break
        }
        let tokens = nafi_lexer::lex(&buffer);
        match tokens {
            Ok(tokens) => println!("{:#?}", tokens),
            Err(e) => eprintln!("{}", e),
        }
    }
}
