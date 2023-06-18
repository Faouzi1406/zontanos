//! The main compiler for **Zontanos**.
//!
//! It takes any zontanos source code and compiles it into machine code.

use std::{fs, io::Write};

use codegen_v2::CodeGen;
use inkwell::context::Context;
use parser_v2::parser::Parser;
use zon_parser::lexer::{Tokenizer, Lexer};

/// The main Ast of the language. It contains the structure of the Ast(Abstract syntax tree) that
/// gets generated after parsing.
pub mod ast;
pub mod codegen;
pub mod codegen_v2;
pub mod parser_v2;
/// The main parser of the language, it parses any given valid Zontanos source code into its
/// Ast.
///
pub mod zon_parser;

pub fn compile(string: String) -> Result<(), &'static str> {
    // Lexing
    let mut lex = Tokenizer::new(&string);
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
