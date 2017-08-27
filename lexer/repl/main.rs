extern crate nafi_lexer;

use std::io;

fn main() {
    let mut buffer = String::with_capacity(80);
    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("IO Failure");
        let line = buffer.trim();
        if line.is_empty() {
            println!("Goodbye!");
            break;
        }
        let tokens = nafi_lexer::lex(line);
        println!("{:#?}", tokens);
    }
}
