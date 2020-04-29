use clap::Clap;
use std::fs;

mod lexer;
use lexer::Lexer;

#[derive(Clap)]
#[clap(version = "1.0", author = "ydolev")]
pub struct Opts {
    file_name: String,
}

pub fn run(opts: Opts) {
    // Read the entire file.
    // TODO(yuval): Better error handling here instead of just calling 'unwrap'.
    let source = fs::read_to_string(opts.file_name).unwrap();

    // Lex the file.
    let mut lexer = Lexer::new(&source);
    lexer.lex();

    println!("Tokens: {:#?}", lexer.tokens);
}
