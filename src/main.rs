use std::fs;
use std::time::Instant;

mod tokenizer;
mod parser;
mod interpreter;
mod cli;

use clap::Parser;
use cli::Cli;

fn main(){
    // let programStart = Instant::now();
    let cli = Cli::parse();
    
    if let Some(filename) = cli.file {
        let source = fs::read_to_string(&filename)
            .unwrap_or_else(|_| panic!("Failed to read the file: {}", filename));

        let tokens = tokenizer::tokenize(&source);
        let ast = parser::parse(&tokens);
        interpreter::interpret(&ast);
    } else{
        eprintln!("Usage: why \"source code\"");
    }
    // let duration = programStart.elapsed();
    
    // println!("");
    // println!("Execution time: {:?}", duration);
}