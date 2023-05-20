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
        {
        let taal: string = \"some string \" 
        let b: u8 = 254 
        let c: char = 'c' 
        let other: string = \"wow\"
        let other: array:char = ['a''b''c']
        let some: array:char =  [ this ]  
        let some: string = function(\"this is pretty cool!\", 10.0, other([120 100 10]))
        }

        fn main(string one, i32 other) f32 {
        fn other(string other) i8 {
        let wow:i32 = 1
        }
            {
                let wow:string = other
            }
        }
        ";

    // Lexing
    let mut lex = Tokenizer::new(&string_vars);
    let lex = Tokenizer::lex(&mut lex);
    println!("{:#?}", lex);

    // Parsing
    let mut parser = Parser::new(lex.clone());
    println!("{:#?}", parser.parse());
}
