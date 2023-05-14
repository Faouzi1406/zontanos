mod all_tests;
mod zon_parser;

use crate::zon_parser::lexer::{self, Lexer, Tokenizer};

fn main() {
    let tokens =
        "==<=\"hello world!\"\n>=<=<>!='h';:>< hello hello_world hello1 let if else pub struct";
    let mut tokenize = lexer::Tokenizer::new(tokens);
    let token = Tokenizer::lex(&mut tokenize);
    println!("{:#?}", token);
}
