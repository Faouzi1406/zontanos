mod all_tests;
mod ast;
mod codegen;
mod parser_v2;
mod zon_parser;

use parser_v2::parser::math::Que;

use crate::{
    codegen::CodeGen,
    zon_parser::{
        lexer::{Lexer, Tokenizer},
        parser::parser::Parser,
    },
};
use std::fs;

fn main() -> Result<(), &'static str> {
    let string_vars = fs::read_to_string("./test_code/main.zon").unwrap_or(
        "Coulnd't read file at ./test_code/main.zon, you are probably not in the root of project."
            .into(),
    );

    // Lexing
    let mut lex = Tokenizer::new(&string_vars);
    let lex = Tokenizer::lex(&mut lex);

    // Parsing
    let mut parser = Parser::new(lex);
    let ast = parser.parse();
    let ast = ast.as_ref().unwrap();

    let code_gen = CodeGen::compile_default(ast);
    let ok = code_gen.unwrap();

    //let create = fs::File::create("./main.l");
    //if let Ok(mut file) = create {
    //  let Ok(_) = file.write(ok.as_bytes()) else {
    //return Err("Coulnd't write output to file");
    //};
    //return Ok(());
    //};

    //let mut stack = Stack::init();
    //stack.push(10);
    //stack.push(40);

    let mut que = Que::init();
    que.append(10);
    que.append(30);
    que.append(50);
    que.append(69);
    que.append(100);
    println!("{:#?}", que);

    return Ok(());
}
