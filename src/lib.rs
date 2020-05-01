use clap::Clap;
use std::fs;

mod ast;

mod lexer;
use lexer::Lexer;

mod parser;
use parser::Parser;

#[derive(Clap)]
#[clap(version = "1.0", author = "ydolev")]
pub struct Opts {
    file_name: String,
}

pub fn run(opts: Opts) {
    // Read the entire file.
    // TODO(yuval): Better error handling here instead of just calling 'unwrap'.
    let source = fs::read_to_string(opts.file_name).unwrap();

    // Lex the program.
    let mut lexer = Lexer::new(&source);
    lexer.lex();

    // Create the global scope.
    let mut global_scope = ast::Scope::new();

    // Parse the program.
    let mut parser = Parser::new(&lexer);
    parser.parse_scope(&mut global_scope);

    println!("Tokens: {:#?}", lexer.tokens);
}
