use crate::zon_parser::lexer::{Tokenize, Tokens, Tokenizer, Lexer};

#[test]
pub fn test_variables() {
    let string_vars = "let a = \"some string \" let b  = 2 let c = 'c'";
    let mut lex = Tokenizer::new(&string_vars);
    let lex = Tokenizer::lex(&mut lex);
    //println!("{}")
}
