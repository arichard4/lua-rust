mod eval;
mod lex;
mod parse;

use std::env;
use std::fs;

fn main() {
    if env::args().len() != 2 {
        println!("Error: Expected one argument, the filename!");
        return;
    }
    let filename = env::args().nth(1).unwrap();
    if !filename.ends_with(".lua") {
        println!("Error: Expected to be given a lua file!");
        return;
    }
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let raw: Vec<char> = contents.chars().collect();

    let tokens = match lex::lex(&raw) {
        Ok(tokens) => tokens,
        Err(msg) => panic!("{}", msg),
    };

    let ast = match parse::parse(&raw, tokens) {
        Ok(ast) => ast,
        Err(msg) => panic!("{}", msg),
    };

    let pgrm = eval::compile(&raw, ast);

    eval::eval(pgrm);
}
