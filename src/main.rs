mod all_tests;
mod ast;
mod zon_parser;

use crate::zon_parser::{
    lexer::{Lexer, Tokenizer},
    parser::parser::{Parse, Parser},
};

fn main() {
    // let file = std::fs::read_to_string("./main.zon").unwrap();
    // let mut lex = Tokenizer::new(&file);
    // let _ = Tokenizer::lex(&mut lex);

    let string_vars = "
        let taal: string = \"some string \" 
        let b: u8 = 254 
        let c: char = 'c' 
        let other: string = \"wow\"";

    let mut lex = Tokenizer::new(&string_vars);
    let lex = Tokenizer::lex(&mut lex);
    let mut parser = Parser::new(lex.clone());
    println!("{:#?}", parser.parse());
}
