mod all_tests;
mod ast;
mod codegen;
mod zon_parser;

use inkwell::context::Context;

use crate::{
    codegen::CodeGen,
    zon_parser::{
        lexer::{Lexer, Tokenizer},
        parser::parser::Parser,
    },
};
use std::fs;

fn main() {
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

    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("main");
    let code_gen = CodeGen {
        builder,
        context: &context,
        module,
    };

    let ast =  ast.unwrap();
    let compiler = code_gen.compile_tree(&ast);
    let ast = compiler;
}
