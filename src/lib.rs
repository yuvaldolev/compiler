use clap::Clap;
use std::error::Error;
use std::fs;

mod lexer;
use lexer::Lexer;

#[derive(Clap)]
#[clap(version = "1.0", author = "ydolev")]
pub struct Opts {
    file_name: String,
}

pub fn run(opts: Opts) -> Result<(), Box<dyn Error>> {
    // Read the entire file.
    let source = fs::read_to_string(opts.file_name)?;

    // Lex the file.
    let lexer = Lexer::new(source, opts.file_name);
    lexer.lex()?;

    Ok(())
}
