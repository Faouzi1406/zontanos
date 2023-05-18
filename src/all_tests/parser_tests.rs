//! This contains all the tests for the parser.

use std::{assert_eq, panic};

use crate::{
    ast::{types::VarTypes, variable::Variable, AstNodeType},
    zon_parser::{
        lexer::{Lexer, Tokenizer},
        parser::parser::{Parse, Parser},
    },
};

#[test]
pub fn test_variables() {
    let string_vars = "let a:string = \"some string \" let b:i32  = 2 let c:char = 'c'";
    let mut lex = Tokenizer::new(&string_vars);
    let lex = Tokenizer::lex(&mut lex);

    let mut parse = Parser::new(lex);
    let parse_tokens = Parser::parse(&mut parse);

    assert!(parse_tokens.is_ok());

    let ast = parse_tokens.unwrap();
    assert_eq!(ast.ast_type, AstNodeType::Program);

    let first_variable = ast.body.get(0);
    assert!(first_variable.is_some());
    assert_eq!(
        first_variable.unwrap(),
        &AstNodeType::Variable(Variable {
            var_name: "a".to_string(),
            var_type: VarTypes::String("some string ".to_string()),
            is_constant: false,
            var_line: 0
        })
    );

    let second_variable = ast.body.get(1);
    assert!(second_variable.is_some());
    assert_eq!(
        second_variable.unwrap(),
        &AstNodeType::Variable(Variable {
            var_name: "b".to_string(),
            var_type: VarTypes::I32(2),
            is_constant: false,
            var_line: 0
        })
    );

    let third_variable = ast.body.get(2);
    assert!(third_variable.is_some());
    assert_eq!(
        third_variable.unwrap(),
        &AstNodeType::Variable(Variable {
            var_name: "c".to_string(),
            var_type: VarTypes::Char('c'),
            is_constant: false,
            var_line: 0
        })
    );
}

#[test]
pub fn test_block() {
    let string_vars = "
        {
        let a:string = \"some string \"
        let b:i32  = 2
        let c:char = 'c'
        }
        ";
    let mut lex = Tokenizer::new(&string_vars);
    let lex = Tokenizer::lex(&mut lex);

    let mut parse = Parser::new(lex);
    let parse_tokens = Parser::parse(&mut parse);

    assert!(parse_tokens.is_ok());

    let ast = parse_tokens.unwrap();
    assert_eq!(ast.ast_type, AstNodeType::Program);

    let block = ast.body.get(0);
    assert!(block.is_some());
    match block.unwrap() {
        AstNodeType::Block(_) => {},
        _ => panic!("Expected block to be of AstNodeType::Block, but it was not")
    }
}
