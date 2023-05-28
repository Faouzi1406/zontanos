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
    parser_v2::ast::{NodeTypes, TypeValues, Types},
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

#[test]
fn parse_single_type() {
    use crate::parser_v2::parser::Parser;

    let statement = "array";
    let mut tokenize = Tokenizer::new(statement);
    let tokenize = Tokenizer::lex(&mut tokenize);

    let mut parser = Parser::new(tokenize);
    let tokens = parser.parse_type_expr().unwrap();
    assert_eq!(tokens.r#type, crate::parser_v2::ast::Types::Array);
}

#[test]
fn parsing_generics() {
    use crate::parser_v2::parser::Parser;

    let statement = "array<i32, string, i32<string, i32<string>>, i32>";
    let mut tokenize = Tokenizer::new(statement);
    let tokenize = Tokenizer::lex(&mut tokenize);

    let mut parser = Parser::new(tokenize);
    let tokens = parser.parse_type_expr().unwrap();
    assert_eq!(tokens.r#type, crate::parser_v2::ast::Types::Array);

    let generic_1 = tokens.generics.get(0).unwrap();
    assert_eq!(generic_1.r#type, crate::parser_v2::ast::Types::I32);

    let generic_2 = tokens.generics.get(1).unwrap();
    assert_eq!(generic_2.r#type, crate::parser_v2::ast::Types::String);

    let generic_2 = tokens.generics.get(2).unwrap();
    assert_eq!(generic_2.r#type, crate::parser_v2::ast::Types::I32);

    let generic_in_generic2 = generic_2.generics.get(0).unwrap();
    assert_eq!(
        generic_in_generic2.r#type,
        crate::parser_v2::ast::Types::String
    );

    let generic_in_generic2 = generic_2.generics.get(1).unwrap();
    assert_eq!(
        generic_in_generic2.r#type,
        crate::parser_v2::ast::Types::I32
    );
}

#[should_panic(expected = "[Parse Error] Expected a end to generics '>' on line 0")]
#[test]
fn parsing_generics_no_end() {
    use crate::parser_v2::parser::Parser;

    let generics = "array<i32, string, i32<string, i32<string>>, i32"; // end of the statement doesn't have a ending '>'
    let mut tokenize = Tokenizer::new(generics);
    let tokenize = Tokenizer::lex(&mut tokenize);

    let mut parser = Parser::new(tokenize);
    let tokens = parser.parse_type_expr();
    panic!("{}", tokens.err().unwrap())
}

#[test]
fn parsing_ident() {
    use crate::parser_v2::parser::Parser;

    let ident = "some";
    let mut tokens = Lexer::new(ident);
    let ident = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(ident);
    let parse = parser.parse_current_ident_expr().unwrap();
    assert_eq!(parse.name, "some")
}

#[should_panic]
#[test]
fn parsing_keyword_not_ident() {
    use crate::parser_v2::parser::Parser;

    let kw = "let";
    let mut tokens = Lexer::new(kw);
    let ident = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(ident);
    let parse = parser.parse_current_ident_expr().unwrap();
    assert_eq!(parse.name, "let")
}

#[test]
fn parsing_values_i8() {
    use crate::parser_v2::parser::Parser;
    let num = "20";
    let mut tokens = Lexer::new(num);
    let ident = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(ident);
    let parse = parser
        .parse_value_expr(crate::parser_v2::ast::Types::I8)
        .unwrap();
    assert_eq!(parse.r#type, TypeValues::I8(20))
}

#[test]
fn parsing_values_i32() {
    use crate::parser_v2::parser::Parser;
    let num = "20";
    let mut tokens = Lexer::new(num);
    let ident = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(ident);
    let parse = parser
        .parse_value_expr(crate::parser_v2::ast::Types::I32)
        .unwrap();
    assert_eq!(parse.r#type, TypeValues::I32(20))
}

#[test]
fn parsing_values_f32() {
    use crate::parser_v2::parser::Parser;
    let float = "20.";
    let mut tokens = Lexer::new(float);
    let ident = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(ident);
    let parse = parser
        .parse_value_expr(crate::parser_v2::ast::Types::F32)
        .unwrap();
    assert_eq!(parse.r#type, TypeValues::F32(20.))
}

#[test]
fn parsing_values_string() {
    use crate::parser_v2::parser::Parser;
    let let_expr = "\"hello world!\"";
    let mut tokens = Lexer::new(let_expr);
    let ident = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(ident);
    let parse = parser
        .parse_value_expr(crate::parser_v2::ast::Types::String)
        .unwrap();
    assert_eq!(parse.r#type, TypeValues::String("hello world!".into()))
}

#[test]
fn parsing_let_expr() {
    use crate::parser_v2::parser::Parser;
    let let_expr = "let test:string = \"testing this\"";
    let mut tokens = Lexer::new(let_expr);
    let var = Tokenizer::lex(&mut tokens);
    //println!("{:#?}", var);
    let mut parser = Parser::new(var);
    let parse = parser.parse_let_expr().unwrap();
    let NodeTypes::Variable(var) = parse.node_type else {
        panic!("Parsing let expr expected the type of node to be a variable")
    };
    assert_eq!(var.ident.name, "test");
    assert_eq!(var.var_type.r#type, Types::String);
}

#[test]
fn parsing_let_exprs() {
    use crate::parser_v2::parser::Parser;
    let ident_str = "let test:string = \"testing this\" 
        let other:string = \"Hello world!\"";
    let mut tokens = Lexer::new(ident_str);
    let var = Tokenizer::lex(&mut tokens); //     println!("tokens {:#?}", var);
    let mut parser = Parser::new(var);
    let parse = parser.parse().unwrap();

    let var1 = parse.body.get(0).unwrap();
    let NodeTypes::Variable(var) = &var1.node_type else {
        panic!("Parsing let exprs expected the type of node to be a variable")
    };
    assert_eq!(var.ident.name, "test");
    assert_eq!(var.var_type.r#type, Types::String);

    let left = var1.left.as_ref().unwrap();
    let NodeTypes::Operator(op) = &left.node_type else {
        panic!("Parsing left exprs expected the type of left node to be a operator")
    };
    assert_eq!(op, &Operator::Eq);

    let right = var1.right.as_ref().unwrap();
    let NodeTypes::Value(op) = &right.node_type else {
        panic!("Parsing right exprs expected the type of right node to be a operator")
    };
    assert_eq!(op.r#type, TypeValues::String("testing this".into()));

    let var2 = parse.body.get(1).unwrap();
    let NodeTypes::Variable(var) = &var2.node_type else {
        panic!("Parsing let exprs expected the type of node to be a variable")
    };
    assert_eq!(var.ident.name, "other");
    assert_eq!(var.var_type.r#type, Types::String);

    let left = var2.left.as_ref().unwrap();
    let NodeTypes::Operator(op) = &left.node_type else {
        panic!("Parsing left exprs expected the type of left node to be a operator")
    };
    assert_eq!(op, &Operator::Eq);

    let right = var2.right.as_ref().unwrap();
    let NodeTypes::Value(op) = &right.node_type else {
        panic!("Parsing right exprs expected the type of right node to be a operator")
    };
    assert_eq!(op.r#type, TypeValues::String("Hello world!".into()));
}
