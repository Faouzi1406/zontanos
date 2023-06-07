// //! This contains all the tests for the parser.
// use std::assert_eq;

use crate::{
    ast::{
        logic::LogicalStatements,
        types::{MarkerTypes, VarTypes},
        variable::Variable,
        Expr,
    },
    panic_test,
    parser_v2::ast::{NodeTypes, Type, TypeValues, Types},
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
    let parse = parser.parse_next_ident_expr().unwrap();
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
    let parse = parser.parse_next_ident_expr().unwrap();
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
        .parse_value_expr(&Type {
            r#type: Types::I8,
            is_array: false,
            is_pointer: false,
            size: 0,
            generics: Vec::new(),
        })
        .unwrap();
    //assert_eq!(parse.value, TypeValues::I8(20))
}

#[test]
fn parsing_values_i32() {
    use crate::parser_v2::parser::Parser;
    let num = "20";
    let mut tokens = Lexer::new(num);
    let ident = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(ident);
    let parse = parser
        .parse_value_expr(&Type {
            r#type: Types::I32,
            is_array: false,
            is_pointer: false,
            size: 0,
            generics: Vec::new(),
        })
        .unwrap();
    println!("{:#?}", parse);
    //assert_eq!(parse.value, TypeValues::I32(20))
}

#[test]
fn parsing_values_f32() {
    use crate::parser_v2::parser::Parser;
    let float = "20.";
    let mut tokens = Lexer::new(float);
    let ident = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(ident);
    let parse = parser
        .parse_value_expr(&Type {
            r#type: Types::F32,
            is_pointer: false,
            is_array: false,
            size: 0,
            generics: Vec::new(),
        })
        .unwrap();
    //assert_eq!(parse.value, TypeValues::F32(20.))
}

#[test]
fn parsing_values_string() {
    use crate::parser_v2::parser::Parser;
    let let_expr = "\"hello world!\"";
    let mut tokens = Lexer::new(let_expr);
    let ident = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(ident);
    let parse = parser
        .parse_value_expr(&Type {
            r#type: Types::String,
            is_array: false,
            is_pointer: false,
            size: 0,
            generics: Vec::new(),
        })
        .unwrap();
    //assert_eq!(parse.value, TypeValues::String("hello world!".into()))
}

#[test]
fn parsing_let_expr() {
    use crate::parser_v2::parser::Parser;
    let let_expr = "let test: string[12] = \"testing this\"";
    let mut tokens = Lexer::new(let_expr);
    let var = Tokenizer::lex(&mut tokens);
    let mut parser = Parser::new(var);
    let parse = parser.parse_let_expr().unwrap();
    let NodeTypes::Variable(var) = parse.node_type else {
        panic!("Parsing let expr expected the type of node to be a variable")
    };
    assert_eq!(var.ident.name, "test");
    assert_eq!(var.var_type.r#type, Types::String);
    assert!(var.var_type.is_array);
}

#[test]
fn parsing_let_exprs() {
    use crate::parser_v2::parser::Parser;
    let ident_str = "let test:string = \"testing this\" 
        let other: string[12] = \"Hello world!\"
        let some: i32[3] = [1, 2, 3]";
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
    assert_eq!(op.value, TypeValues::String("testing this".into()));

    let var2 = parse.body.get(1).unwrap();
    let NodeTypes::Variable(var) = &var2.node_type else {
        panic!("Parsing let exprs expected the type of node to be a variable")
    };
    assert!(var.var_type.is_array);
    assert_eq!(var.var_type.size, 12);
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
    assert_eq!(op.value, TypeValues::String("Hello world!".into()));

    let var3 = parse.body.get(2).unwrap();
    let NodeTypes::Variable(var) = &var3.node_type else {
        panic!("Parsing let exprs expected the type of node to be a variable")
    };
    assert_eq!(var.ident.name, "some");
    assert_eq!(var.var_type.r#type, Types::I32);
    assert!(var.var_type.is_array);

    let right = var3.right.as_ref().unwrap();
    let NodeTypes::Value(op) = &right.node_type else {
        panic!("Parsing right exprs expected the type of right node to be a operator")
    };
    assert_eq!(
        op.value,
        TypeValues::Array(vec![
            TypeValues::I32(1),
            TypeValues::I32(2),
            TypeValues::I32(3)
        ])
    );
}

#[test]
fn parsing_arrays() {
    use crate::parser_v2::parser::Parser;
    let type_array = "char[3]";
    let values = "['a', 'b', 'c']";

    let mut type_array = Lexer::new(type_array);
    let type_array = Tokenizer::lex(&mut type_array);
    let mut parser = Parser::new(type_array);
    let type_array = parser.parse_type_expr().unwrap();

    let mut tokens = Lexer::new(values);
    let var = Tokenizer::lex(&mut tokens);

    let mut parser = Parser::new(var);
    let parse = parser.parse_array(&type_array).unwrap();
    assert_eq!(parse.get(0), Some(&TypeValues::Char('a')));
    assert_eq!(parse.get(1), Some(&TypeValues::Char('b')));
    assert_eq!(parse.get(2), Some(&TypeValues::Char('c')));
}

#[test]
fn parsing_paramaters() {
    use crate::parser_v2::parser::Parser;
    let params = "(hello: string, other: string, some: array<i32>)";

    let mut params = Lexer::new(params);
    let params = Tokenizer::lex(&mut params);
    let mut parser = Parser::new(params);
    let params = parser.parse_params().expect("Couldn't unwrap on params?");

    let first_param = &params
        .get(0)
        .expect("Couldn't get the first paramater")
        .r#type;
    assert_eq!(first_param.r#type, Types::String);

    let second_param = &params.get(0).expect("Couldn't get the second paramater");
    let name = &second_param.ident;
    assert_eq!(name.name, "hello");
    assert_eq!(second_param.r#type.r#type, Types::String);

    let second_param = &params.get(1).expect("Couldn't get the second paramater");
    let name = &second_param.ident;
    assert_eq!(name.name, "other");
    assert_eq!(second_param.r#type.r#type, Types::String);

    let second_param = &params.get(2).expect("Couldn't get the second paramater");
    let name = &second_param.ident;
    assert_eq!(name.name, "some");
    assert_eq!(second_param.r#type.r#type, Types::Array);
    let generic_i32 = second_param.r#type.generics.get(0).unwrap();
    assert_eq!(generic_i32.r#type, Types::I32);
}

#[test]
fn parsing_functions() {
    use crate::parser_v2::parser::Parser;
    let params = "fn name(hello: string, other: string, some: array<i32>) string {
        let some: string = \"hello world!\";
    }";

    let mut params = Lexer::new(params);
    let params = Tokenizer::lex(&mut params);
    let mut parser = Parser::new(params);
    let function = parser.parse().expect("Coudln't parse function");

    let function = function
        .body
        .get(0)
        .expect("Couldn't get node in side body of ast");
    let NodeTypes::Function(function) = &function.node_type else {
        panic!("Expected the firtst nody in the ast body to be a function (when parsing a function).");
    };

    assert_eq!(function.returns.r#type, Types::String);

    let function_let = function.body.get(0).unwrap();
    let NodeTypes::Variable(func) = &function_let.node_type else {
        panic!("Expected the firt node type in the function body to be of variable 'let some: string = ...';");
    };
    assert_eq!(func.ident.name, "some");
    assert_eq!(func.var_type.r#type, Types::String);
}

#[test]
fn parse_arguments() {
    use crate::parser_v2::parser::Parser;
    let args = "(1, 10, \"testing\")";

    let mut params = Tokenizer::new(args);
    let tokens = Tokenizer::lex(&mut params);
    let mut parser = Parser::new(tokens);
    let parse = parser.parse_args_expr().unwrap();

    let first_arg = parse.get(0).unwrap();
    assert_eq!(first_arg.value, TypeValues::I32(1));

    let second_arg = parse.get(1).unwrap();
    assert_eq!(second_arg.value, TypeValues::I32(10));

    let second_arg = parse.get(2).unwrap();
    assert_eq!(second_arg.value, TypeValues::String("testing".into()));
}

#[test]
fn parse_block_expr() {
    use crate::parser_v2::parser::Parser;
    let args = "{
        let some: char = 'a'
        func(some)
        return 10
    }";

    let mut params = Tokenizer::new(args);
    let tokens = Tokenizer::lex(&mut params);
    let mut parser = Parser::new(tokens);
    let (body, _) = parser
        .parse_block_expr(&Type {
            r#type: Types::I32,
            is_array: false,
            is_pointer: false,
            size: 0,
            generics: Vec::new(),
        })
        .unwrap();

    let NodeTypes::Variable(value) = &body.get(0).unwrap().node_type else {
        panic!("Couldn't turn value in block into variable");
    };
    assert_eq!(value.ident.name, "some".to_string());
    assert_eq!(value.var_type.r#type, Types::Char);

    let value = &body.get(1).unwrap();

    let NodeTypes::FunctionCall(call) = &value.node_type else {
        panic!("Couldn't turn value in block into function call");
    };
    assert_eq!(call.calls_to.name, "func");

    let args = &value.left.as_ref().unwrap();
    let NodeTypes::Arguments(args) = &args.node_type else {
        panic!("Couldn't turn value in block into function call");
    };
    let arg_1 = args.get(0).unwrap();
    assert_eq!(arg_1.value, TypeValues::Identifier("some".into()));

    let NodeTypes::Return = &body.get(2).unwrap().node_type else {
        panic!("Couldn't turn value in block into return value");
    };
    //assert_eq!(value.value, TypeValues::I32(10))
}

#[test]
fn parse_func_call() {
    use crate::parser_v2::parser::Parser;
    let args = "test(1, 10, \"testing\")";

    let mut params = Tokenizer::new(args);
    let tokens = Tokenizer::lex(&mut params);
    let mut parser = Parser::new(tokens);
    let parse = parser.parse_fn_call_expr().unwrap();

    let first_arg = parse.0.calls_to.name;
    assert_eq!(first_arg, "test".to_string());

    let NodeTypes::Arguments(arguments) = parse.1 else {
        panic!("parse function call expected arguments");
    };

    let firt_arg = arguments.get(0).unwrap();
    assert_eq!(firt_arg.value, TypeValues::I32(1));

    let firt_arg = arguments.get(1).unwrap();
    assert_eq!(firt_arg.value, TypeValues::I32(10));

    let firt_arg = arguments.get(2).unwrap();
    assert_eq!(firt_arg.value, TypeValues::String("testing".into()))
}

#[test]
fn parse_statements() {
    use crate::parser_v2::parser::Parser;
    // the closecurrlybrace '{' serves for the end of a logical statements
    let statements = " 10 > 20 || 20 == 10 {";

    let mut statements = Tokenizer::new(statements);
    let statements_tokens = Tokenizer::lex(&mut statements);
    let mut parser = Parser::new(statements_tokens);
    let parse_statements = parser.lep_parse_statements().unwrap();
}
