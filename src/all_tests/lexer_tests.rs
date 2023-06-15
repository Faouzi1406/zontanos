use crate::{
    panic_test,
    zon_parser::lexer::{Keywords, Lexer, Operator, Tokenizer, Tokens},
};

#[test]
pub fn test_more_tokens() {
    let str = "> >= >=>";
    let mut tokenizer = Tokenizer::new(str);
    let lexer = Tokenizer::lex(&mut tokenizer);

    let Some(first_more) = lexer.first() else {
        panic_test!("Lexer test more tokens", "Error expected the first token of of lexer tokens to be Some(More) but got None");
    };
    assert_eq!(first_more.token_type, Tokens::Op(Operator::More));

    let Some(second_token) = lexer.get(1) else {
        panic_test!("Lexer test more tokens", "Error expected the second token of of lexer tokens to be Some(MoreEq) but got None");
    };
    assert_eq!(second_token.token_type, Tokens::Op(Operator::MoreEq));

    let Some(third_token) = lexer.get(2) else {
        panic_test!("Lexer test more tokens", "Error expected the third token of of lexer tokens to be Some(MoreEq) but got None");
    };
    assert_eq!(third_token.token_type, Tokens::Op(Operator::MoreEq));

    let Some(fourth_token) = lexer.get(3) else {
        panic_test!("Lexer test more tokens", "Error expected the fourth token of of lexer tokens to be Some(More) but got None");
    };
    assert_eq!(fourth_token.token_type, Tokens::Op(Operator::More));
}

#[test]
pub fn test_less_tokens() {
    let str = "< <= <=<";
    let mut tokenizer = Tokenizer::new(str);
    let lexer = Tokenizer::lex(&mut tokenizer);

    let Some(first_more) = lexer.first() else {
        panic_test!("Lexer test more tokens", "Error expected the first token of of lexer tokens to be Some(Less) but got None");
    };
    assert_eq!(first_more.token_type, Tokens::Op(Operator::Less));

    let Some(second_token) = lexer.get(1) else {
        panic_test!("Lexer test more tokens", "Error expected the second token of of lexer tokens to be Some(LessEq) but got None");
    };
    assert_eq!(second_token.token_type, Tokens::Op(Operator::LessEq));

    let Some(third_token) = lexer.get(2) else {
        panic_test!("Lexer test more tokens", "Error expected the third token of of lexer tokens to be Some(LessEq) but got None");
    };
    assert_eq!(third_token.token_type, Tokens::Op(Operator::LessEq));

    let Some(fourth_token) = lexer.get(3) else {
        panic_test!("Lexer test more tokens", "Error expected the fourth token of of lexer tokens to be Some(Less) but got None");
    };
    assert_eq!(fourth_token.token_type, Tokens::Op(Operator::Less));
}

#[test]
pub fn test_eq_tokens() {
    let str = "= == ===";
    let mut tokenizer = Tokenizer::new(str);
    let lexer = Tokenizer::lex(&mut tokenizer);

    let Some(first_more) = lexer.first() else {
        panic_test!("Lexer test more tokens", "Error expected the first token of of lexer tokens to be Some(Eq) but got None");
    };
    assert_eq!(first_more.token_type, Tokens::Op(Operator::Eq));

    let Some(second_token) = lexer.get(1) else {
        panic_test!("Lexer test more tokens", "Error expected the second token of of lexer tokens to be Some(EqEq) but got None");
    };
    assert_eq!(second_token.token_type, Tokens::Op(Operator::EqEq));

    let Some(third_token) = lexer.get(2) else {
        panic_test!("Lexer test more tokens", "Error expected the third token of of lexer tokens to be Some(EqEq) but got None");
    };
    assert_eq!(third_token.token_type, Tokens::Op(Operator::EqEq));

    let Some(fourth_token) = lexer.get(3) else {
        panic_test!("Lexer test more tokens", "Error expected the fourth token of of lexer tokens to be Some(Eq) but got None");
    };
    assert_eq!(fourth_token.token_type, Tokens::Op(Operator::Eq));
}

#[test]
pub fn test_nq_bang_tokens() {
    let str = "! != !=!";
    let mut tokenizer = Tokenizer::new(str);
    let lexer = Tokenizer::lex(&mut tokenizer);

    let Some(first_more) = lexer.first() else {
        panic_test!("Lexer test more tokens", "Error expected the first token of of lexer tokens to be Some(Bang) but got None");
    };
    assert_eq!(first_more.token_type, Tokens::Bang);

    let Some(second_token) = lexer.get(1) else {
        panic_test!("Lexer test more tokens", "Error expected the second token of of lexer tokens to be Some(Nq) but got None");
    };
    assert_eq!(second_token.token_type, Tokens::Op(Operator::Nq));

    let Some(third_token) = lexer.get(2) else {
        panic_test!("Lexer test more tokens", "Error expected the third token of of lexer tokens to be Some(Nq) but got None");
    };
    assert_eq!(third_token.token_type, Tokens::Op(Operator::Nq));

    let Some(fourth_token) = lexer.get(3) else {
        panic_test!("Lexer test more tokens", "Error expected the fourth token of of lexer tokens to be Some(Bang) but got None");
    };
    assert_eq!(fourth_token.token_type, Tokens::Bang);
}

#[test]
pub fn test_string_and_numbers() {
    let str = "\"Hello world!\" 123456 12.12.12 12_12_12 120000\"Hello world!\" -10";
    let mut tokenizer = Tokenizer::new(str);
    let lexer = Tokenizer::lex(&mut tokenizer);

    let Some(first_more) = lexer.first() else {
        panic_test!("Lexer test more tokens", "Error expected the first token of of lexer tokens to be Some(String) but got None");
    };
    assert_eq!(first_more.token_type, Tokens::String);
    assert_eq!(first_more.value, "Hello world!");

    let Some(second_token) = lexer.get(1) else {
        panic_test!("Lexer test more tokens", "Error expected the second token of of lexer tokens to be Some(Number) but got None");
    };
    assert_eq!(second_token.token_type, Tokens::Number);
    assert_eq!(second_token.value, "123456");

    let Some(third_token) = lexer.get(2) else {
        panic_test!("Lexer test more tokens", "Error expected the third token of of lexer tokens to be Some(FloatNumber) but got None");
    };
    assert_eq!(third_token.token_type, Tokens::FloatNumber);
    assert_eq!(third_token.value, "12.12.12");

    let Some(fourth_token) = lexer.get(3) else {
        panic_test!("Lexer test more tokens", "Error expected the fourth token of of lexer tokens to be Some(Bang) but got None");
    };
    assert_eq!(fourth_token.token_type, Tokens::Number);
    assert_eq!(fourth_token.value, "121212");

    let Some(fifth_token) = lexer.get(4) else {
        panic_test!("Lexer test more tokens", "Error expected the fifth token of of lexer tokens to be Some(Number) but got None");
    };
    assert_eq!(fifth_token.token_type, Tokens::Number);
    assert_eq!(fifth_token.value, "120000");

    let Some(fifth_token) = lexer.get(5) else {
        panic_test!("Lexer test more tokens", "Error expected the fifth token of of lexer tokens to be Some(String) but got None");
    };
    assert_eq!(fifth_token.token_type, Tokens::String);
    assert_eq!(fifth_token.value, "Hello world!");

    let Some(fifth_token) = lexer.get(6) else {
        panic_test!("Lexer test more tokens", "Error expected the fifth token of of lexer tokens to be Some(String) but got None");
    };
    assert_eq!(fifth_token.token_type, Tokens::NegativeNumber);
    assert_eq!(fifth_token.value, "10");
}

#[test]
pub fn test_keywords() {
    let str = "let if else for struct pub enum";
    let mut tokenizer = Tokenizer::new(str);
    let lexer = Tokenizer::lex(&mut tokenizer);

    let Some(first_more) = lexer.first() else {
        panic_test!("Lexer test more tokens", "Error expected the first token of of lexer tokens to be Some(Kw) but got None");
    };
    assert_eq!(first_more.token_type, Tokens::Kw(Keywords::Let));
    assert_eq!(first_more.value, "let");

    let Some(second_token) = lexer.get(1) else {
        panic_test!("Lexer test more tokens", "Error expected the second token of of lexer tokens to be Some(Kw) but got None");
    };
    assert_eq!(second_token.token_type, Tokens::Kw(Keywords::If));
    assert_eq!(second_token.value, "if");

    let Some(third_token) = lexer.get(2) else {
        panic_test!("Lexer test more tokens", "Error expected the third token of of lexer tokens to be Some(Kw) but got None");
    };
    assert_eq!(third_token.token_type, Tokens::Kw(Keywords::Else));
    assert_eq!(third_token.value, "else");

    let Some(fourth_token) = lexer.get(3) else {
        panic_test!("Lexer test more tokens", "Error expected the fourth token of of lexer tokens to be Some(Kw) but got None");
    };
    assert_eq!(fourth_token.token_type, Tokens::Kw(Keywords::For));
    assert_eq!(fourth_token.value, "for");

    let Some(fifth_token) = lexer.get(4) else {
        panic_test!("Lexer test more tokens", "Error expected the fifth token of of lexer tokens to be Some(Kw) but got None");
    };
    assert_eq!(fifth_token.token_type, Tokens::Kw(Keywords::Struct));
    assert_eq!(fifth_token.value, "struct");

    let Some(fifth_token) = lexer.get(5) else {
        panic_test!("Lexer test more tokens", "Error expected the fifth token of of lexer tokens to be Some(Kw) but got None");
    };
    assert_eq!(fifth_token.token_type, Tokens::Kw(Keywords::Pub));
    assert_eq!(fifth_token.value, "pub");

    let Some(sixth_token) = lexer.get(6) else {
        panic_test!("Lexer test more tokens", "Error expected the fifth token of of lexer tokens to be Some(Kw) but got None");
    };
    assert_eq!(sixth_token.token_type, Tokens::Kw(Keywords::Enum));
    assert_eq!(sixth_token.value, "enum");
}

#[test]
pub fn test_identifiers() {
    let str = "hello hello_world hello0";
    let mut tokenizer = Tokenizer::new(str);
    let lexer = Tokenizer::lex(&mut tokenizer);

    let Some(first_token) = lexer.first() else {
        panic_test!("Lexer test more tokens", "Error expected the first token of of lexer tokens to be Some(Identifier) but got None");
    };
    assert_eq!(first_token.token_type, Tokens::Identifier);
    assert_eq!(first_token.value, "hello");

    let Some(second_token) = lexer.get(1) else {
        panic_test!("Lexer test more tokens", "Error expected the second token of of lexer tokens to be Some(Identifier) but got None");
    };
    assert_eq!(second_token.token_type, Tokens::Identifier);
    assert_eq!(second_token.value, "hello_world");

    let Some(second_token) = lexer.get(2) else {
        panic_test!("Lexer test more tokens", "Error expected the second token of of lexer tokens to be Some(Identifier) but got None");
    };
    assert_eq!(second_token.token_type, Tokens::Identifier);
    assert_eq!(second_token.value, "hello0");
}

#[test]
pub fn test_syntax() {
    let str = "fn add(u32 a, u32 b) { let a = b; return a; } fn main() {}";

    let mut tokenizer = Tokenizer::new(str);
    let lexer = Tokenizer::lex(&mut tokenizer);

    let Some(first_token) = lexer.first() else {
        panic_test!("Lexer test more tokens", "Error expected the first token of of lexer tokens to be Some(Kw) but got None");
    };
    assert_eq!(first_token.token_type, Tokens::Kw(Keywords::Fn));
    assert_eq!(first_token.value, "fn");

    let Some(second_token) = lexer.get(1) else {
        panic_test!("Lexer test more tokens", "Error expected the second token of of lexer tokens to be Some(Identifier) but got None");
    };
    assert_eq!(second_token.token_type, Tokens::Identifier);
    assert_eq!(second_token.value, "add");

    let Some(third_token) = lexer.get(2) else {
        panic_test!("lexer test more tokens", "error expected the third token of of lexer tokens to be some(openbrace) but got none");
    };
    assert_eq!(third_token.token_type, Tokens::OpenBrace);
    assert_eq!(third_token.value, "(");

    let Some(fourth_token) = lexer.get(3) else {
        panic_test!("lexer test more tokens", "error expected the fourth token of of lexer tokens to be some(Identifier) but got none");
    };
    assert_eq!(fourth_token.token_type, Tokens::Identifier);
    assert_eq!(fourth_token.value, "u32");

    let Some(fifth_token) = lexer.get(4) else {
        panic_test!("lexer test more tokens", "error expected the fifth token of of lexer tokens to be some(Identifier) but got none");
    };
    assert_eq!(fifth_token.token_type, Tokens::Identifier);
    assert_eq!(fifth_token.value, "a");

    let Some(sixth_token) = lexer.get(5) else {
        panic_test!("lexer test more tokens", "error expected the sixth token of of lexer tokens to be some(Identifier) but got none");
    };
    assert_eq!(sixth_token.token_type, Tokens::Comma);
    assert_eq!(sixth_token.value, ",");

    let Some(eight_token) = lexer.get(8) else {
        panic_test!("lexer test more tokens", "error expected the 8th token of of lexer tokens to be some(Identifier) but got none");
    };
    assert_eq!(eight_token.token_type, Tokens::CloseBrace);
    assert_eq!(eight_token.value, ")");

    let Some(let_token) = lexer.get(10) else {
        panic_test!("lexer test more tokens", "error expected the 9th  of of lexer tokens to be some(Kw) but got none");
    };
    assert_eq!(let_token.token_type, Tokens::Kw(Keywords::Let));
    assert_eq!(let_token.value, "let");
}
