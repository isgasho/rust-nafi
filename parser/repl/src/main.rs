use ron::ser::to_string_pretty;
use std::io::{self, prelude::*};

fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();
    loop {
        write!(stdout, "? ")?;
        stdout.flush()?;
        let input = {
            let mut buf = String::new();
            if stdin.read_line(&mut buf)? == 0 {
                return Ok(());
            }
            buf
        };
        if input.starts_with(";;") {
            let command = input[2..].trim();
            match command {
                "help" => writeln!(stdout, "help TODO")?,
                "exit" => return Ok(()),
                _ => writeln!(stdout, "Unsupported command; try `;;help`")?,
            };
        } else {
            match ::nafi_parser::parse(&input) {
                Ok(parse) => writeln!(
                    stdout,
                    "{}",
                    to_string_pretty(&parse, Default::default()).unwrap()
                )?,
                Err(err) => writeln!(stdout, "{}", err)?,
            };
        }
    }
}

fn main() {
    run().unwrap()
}
