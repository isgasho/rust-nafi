extern crate nafi_lexer;
use std::io;

fn main() {
    let mut buffer = String::with_capacity(80);
    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("IO Failure");
        if buffer.is_empty() {
            break;
        }
        let tokens = nafi_lexer::lex(buffer.trim());
        println!("{:?}", tokens);
    }
}
