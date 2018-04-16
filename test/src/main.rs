#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;
extern crate ansi_term;
extern crate difference;
extern crate env_logger;
extern crate log;
extern crate rayon;
extern crate walkdir;

extern crate nafi_lexer as lexer;
extern crate nafi_lexer_repl as lexer_harness;

type Result<T> = ::std::result::Result<T, failure::Error>;
use log::LevelFilter;
use structopt::StructOpt;

mod format_changeset;
mod fs;

mod lexer_fuzz_tests;
mod lexer_tests;

fn run(opts: Opts) -> Result<()> {
    let all = opts.all();

    if all || opts.lexer {
        println!("lexer tests");
        println!("{}", "=".repeat(80));
        lexer_tests::test()?;
        println!("lexer tests were successful!");
        println!();
    }

    if all || opts.lexer_fuzz {
        println!("lexer fuzz tests");
        println!("{}", "=".repeat(80));
        lexer_fuzz_tests::test()?;
        println!("lexer fuzz tests were successful");
        println!();
    }

    Ok(())
}

/// Test harness for structural tests of the rust-nafi project.
///
/// If no arguments are provided, all tests are run.
#[derive(Copy, Clone, Debug, StructOpt)]
#[repr(packed)]
struct Opts {
    /// Run tests for source->token transformation
    #[structopt(long = "lexer")]
    lexer: bool,
    /// Run simple fuzz tests over the lexer
    #[structopt(long = "lexer-fuzz")]
    lexer_fuzz: bool,
}

impl Opts {
    fn all(&self) -> bool { !(self.lexer || self.lexer_fuzz) }
}

fn main() {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap_or_else(|_| eprintln!("Color support unavailable"));

    fn run() -> Result<()> {
        let opts = Opts::from_args();
        env_logger::Builder::new()
            .filter(Some(env!("CARGO_PKG_NAME")), LevelFilter::Warn)
            .filter(None, LevelFilter::Off)
            .try_init()?;
        ::run(opts)?;
        Ok(())
    }

    match run() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    }
}
