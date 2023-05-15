mod all_tests;
mod ast;
mod zon_parser;

use crate::zon_parser::lexer::{Lexer, Tokenizer};

fn main() {
    let file = std::fs::read_to_string("./main.zon").unwrap();
    let mut lex = Tokenizer::new(&file);
    let lex = Tokenizer::lex(&mut lex);
    println!("{:#?}", lex);
}
