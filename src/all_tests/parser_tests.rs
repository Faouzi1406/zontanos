//! This contains all the tests for the parser.

use std::assert_eq;

use crate::{
    ast::{
        logic::LogicalStatements,
        types::{MarkerTypes, VarTypes},
        variable::Variable,
        Expr,
    },
    panic_test,
    zon_parser::{
        lexer::{Lexer, Operator, Tokenizer},
        parser::parser::Parser,
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
    assert_eq!(ast.ast_type, Expr::Program);

    let first_variable = ast.body.get(0);
    assert!(first_variable.is_some());
    assert_eq!(
        first_variable.unwrap(),
        &Expr::Variable(Variable {
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
        &Expr::Variable(Variable {
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
        &Expr::Variable(Variable {
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
    assert_eq!(ast.ast_type, Expr::Program);

    let block = ast.body.get(0);
    assert!(block.is_some());
    match block.unwrap() {
        Expr::Block(_) => {}
        _ => panic!("Expected block to be of AstNodeType::Block, but it was not"),
    }
}

#[test]
fn parsing_function() {
    let function = "fn print_name() i32 { print(\"name\") }";
    let mut tokenize = Tokenizer::new(function);
    let tokenize = Tokenizer::lex(&mut tokenize);
    let mut parse = Parser::new(tokenize);
    let Ok(parse) = parse.parse() else {
        panic_test!("parsing function", "expected the ast to be ok but it wasn't");
    };
    let Some(token) = parse.body.get(0) else {
        panic_test!("parsing function", "expected first node to be some but it was None");
    };
    let Expr::Function(token) = token  else {
        panic_test!("parsing function", "expected first node to be function but it wasn't");
    };
    assert_eq!(token.name, "print_name");
    assert_eq!(token.return_type, MarkerTypes::I32);

    let Some(body) = &token.block.body.get(0) else {
        panic_test!("parsing function", "expected the function body to be ok but it wasn't");
    };
    let Expr::FunctionCall(call) = body  else {
        panic_test!("parsing function", "expected the function body to containt a function call but it didn't");
    };
    assert_eq!(call.call_to, "print");
}

#[test]
fn parsing_function_calls() {
    let function_call = "{ print(\"name\") }";

    let mut tokenize = Tokenizer::new(function_call);
    let tokenize = Tokenizer::lex(&mut tokenize);
    let mut parse = Parser::new(tokenize);
    let Ok(parse) = parse.parse() else {
        panic_test!("parsing function calls", "expected the ast to be ok but it wasn't");
    };
    let Some(token) = parse.body.get(0) else {
        panic_test!("parsing function", "expected first node to be some but it was None");
    };
    let Expr::Block(block) = token  else {
        panic_test!("parsing function calls", "expected first node to be block but it wasn't");
    };
    let Some(body) = block.body.get(0) else {
        panic_test!("parsing function calls", "expected the function body to be ok but it wasn't");
    };
    let Expr::FunctionCall(call) = body  else {
        panic_test!("parsing function calls", "expected the function body to contain a function call but it didn't");
    };
    assert_eq!(call.call_to, "print");
    let args = &call.args;
    let Some(first_arg) = args.get(0) else {
        panic_test!("parsing function calls", "expected the function call to contain a a argument but it didn't");
    };
    let VarTypes::String(name) = first_arg else {
        panic_test!("parsing function calls", "expected the function body to containt a function call but it didn't");
    };
    assert_eq!(name, "name");
}

#[test]
fn parsing_statements() {
    let statement = "if ( 1 > 2 || 2 < 1 && 2 == 20 ) {
        print(\"some\", [1 2 3])
    }";
    let mut tokenize = Tokenizer::new(statement);
    let tokenize = Tokenizer::lex(&mut tokenize);
    let mut parse = Parser::new(tokenize);
    let Ok(parse) = parse.parse() else {
        panic_test!("parsing statements", "expected the ast to be ok but it wasn't");
    };
    let Some(token) = parse.body.get(0) else {
        panic_test!("parsing statements", "expected first token to be some but it was None");
    };
    let Expr::Logic(logic) = token else {
        panic_test!("parsing statements", "expected first token to be a logic token but it wasn't");
    };
    let statements = &logic.statements;

    let Some(first) = statements.get(0) else {
        panic_test!("parsing statements", "expected first statement to be a logic token but it wasn't");
    };
    match first {
        LogicalStatements::Or(one, two) => {
            assert_eq!(one.value_1, VarTypes::I32(1));
            assert_eq!(one.operator, Some(Operator::More));
            assert_eq!(two.value_1, VarTypes::I32(2));
            assert_eq!(two.operator, Some(Operator::Less));
        }
        statement => panic!("expected first statement to be a Or but it was a {statement:#?}"),
    };
}
