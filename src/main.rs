#![allow(unused)]
mod all_tests;
mod ast;
mod codegen;
mod parser_v2;
mod zon_parser;

use inkwell::context::Context;
use parser_v2::parser::math::Stack;
use zontanos::codegen_v2::CodeGen;
use zontanos::parser_v2::parser::Parser;

use std::io::Write;
use std::{fs, process::exit};
use zontanos::zon_parser::lexer::{Lexer, Tokenizer};

fn main() -> Result<(), &'static str> {
    let string_vars = fs::read_to_string("./test_code/main.zon").unwrap_or(
        "Coulnd't read file at ./test_code/main.zon, you are probably not in the root of project."
            .into(),
    );

    // Lexing
    let mut lex = Tokenizer::new(&string_vars);
    let lex = Tokenizer::lex(&mut lex);

    let mut parser = Parser::new(lex);
    let ast = parser.parse();
    let ast = ast.unwrap();

    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("main");
    let mut codegen = CodeGen {
        module,
        builder,
        context: &context,
        scope: None,
    };

    let code_gen = codegen.compile_ast(&ast);
    code_gen.unwrap();

    let create = fs::File::create("./main.l");
    if let Ok(mut file) = create {
        let Ok(_) = file.write(codegen.module.to_string().as_bytes()) else {
        return Err("Coulnd't write output to file");
    };
        return Ok(());
    };

    Ok(())
}
