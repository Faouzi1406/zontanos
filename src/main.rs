mod all_tests;
mod ast;
mod zon_parser;

use std::fs;

use crate::zon_parser::{
    lexer::{Lexer, Tokenizer},
    parser::parser::Parser,
};

fn main() {
    // let file = std::fs::read_to_string("./main.zon").unwrap();
    // let mut lex = Tokenizer::new(&file);
    // let _ = Tokenizer::lex(&mut lex);

    let string_vars = fs::read_to_string("./test_code/main.zon").unwrap_or(
        "Coulnd't read file at ./test_code/main.zon, you are probably not in the root of project."
            .into(),
    );

    // Lexing
    let mut lex = Tokenizer::new(&string_vars);
    let lex = Tokenizer::lex(&mut lex);
    println!("{:#?}", lex);

    // Parsing
    let mut parser = Parser::new(lex.clone());
    let ast = parser.parse();
    println!("{:#?}", ast);
}
