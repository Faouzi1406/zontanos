mod all_tests;
mod ast;
mod codegen;
mod zon_parser;

use crate::{
    codegen::CodeGen,
    zon_parser::{
        lexer::{Lexer, Tokenizer},
        parser::parser::Parser,
    },
};
use std::{fs, io::Write};

fn main() -> Result<(), &'static str> {
    let string_vars = fs::read_to_string("./test_code/main.zon").unwrap_or(
        "Coulnd't read file at ./test_code/main.zon, you are probably not in the root of project."
            .into(),
    );

    // Lexing
    let mut lex = Tokenizer::new(&string_vars);
    let lex = Tokenizer::lex(&mut lex);

    // Parsing
    let mut parser = Parser::new(lex.clone());
    let ast = parser.parse();
    println!("{:#?}", ast);
    let ast = ast.as_ref().unwrap();

    let code_gen = CodeGen::compile_default(ast);
    let ok = code_gen.unwrap();

    let create = fs::File::create("./main.l");
    if let Ok(mut file) = create {
        let Ok(write) = file.write(ok.as_bytes()) else {
            return Err("Coulnd't write output to file");
        };
        return Ok(());
    };
    println!("{create:#?}");
    return Err("Coulnd't create file and compile");
}
