use super::lexer::{Keywords, Operator, Tokens};
use crate::zon_parser::lexer::TokenErrorMessages;

impl From<&str> for Tokens {
    fn from(value: &str) -> Self {
        match value {
            // All normal token values
            "!" => Tokens::Bang,
            "/" => Tokens::Slash,
            "(" => Tokens::OpenBrace,
            ")" => Tokens::CloseBrace,
            "{" => Tokens::OpenCurlyBracket,
            "}" => Tokens::CloseCurlyBracket,
            "[" => Tokens::OpenBracket,
            "]" => Tokens::CloseBracket,
            "^" => Tokens::Pointer,
            ":" => Tokens::Colon,
            ";" => Tokens::SemiColon,
            "+" => Tokens::Op(Operator::Plus),
            "-" => Tokens::Op(Operator::Min),
            "*" => Tokens::Op(Operator::Times),
            "," => Tokens::Comma,
            "\t" => Tokens::Tab,
            "true" => Tokens::BoolTrue,
            "false" => Tokens::BoolFalse,
            // All operator values
            "=" => Tokens::Op(Operator::Eq),
            "&" => Tokens::Op(Operator::And),
            ">" => Tokens::Op(Operator::More),
            "<" => Tokens::Op(Operator::Less),
            "==" => Tokens::Op(Operator::EqEq),
            "!=" => Tokens::Op(Operator::Nq),
            ">=" => Tokens::Op(Operator::MoreEq),
            "<=" => Tokens::Op(Operator::LessEq),
            "&&" => Tokens::Op(Operator::AndAnd),
            "||" => Tokens::Op(Operator::OrOr),
            // All Keywords
            "return" => Tokens::Kw(Keywords::Return),
            "let" => Tokens::Kw(Keywords::Let),
            "fn" => Tokens::Kw(Keywords::Fn),
            "for" => Tokens::Kw(Keywords::For),
            "if" => Tokens::Kw(Keywords::If),
            "else" => Tokens::Kw(Keywords::Else),
            "struct" => Tokens::Kw(Keywords::Struct),
            "enum" => Tokens::Kw(Keywords::Enum),
            "pub" => Tokens::Kw(Keywords::Pub),
            "string" => Tokens::Kw(Keywords::String),
            "char" => Tokens::Kw(Keywords::Char),
            "array" => Tokens::Kw(Keywords::Array),
            "void" => Tokens::Kw(Keywords::Void),
            "i32" => Tokens::Kw(Keywords::I32),
            "f32" => Tokens::Kw(Keywords::F32),
            "u8" => Tokens::Kw(Keywords::U8),
            "i8" => Tokens::Kw(Keywords::I8),
            // Every other value found will be seen as a Identifier
            _ => Tokens::Identifier,
        }
    }
}

impl From<char> for Tokens {
    fn from(value: char) -> Self {
        match value {
            // All normal token values
            '!' => Tokens::Bang,
            '/' => Tokens::Slash,
            '(' => Tokens::OpenBrace,
            ')' => Tokens::CloseBrace,
            '{' => Tokens::OpenCurlyBracket,
            '}' => Tokens::CloseCurlyBracket,
            '[' => Tokens::OpenBracket,
            ']' => Tokens::CloseBracket,
            '^' => Tokens::Pointer,
            ':' => Tokens::Colon,
            '+' => Tokens::Op(Operator::Plus),
            '-' => Tokens::Op(Operator::Min),
            '*' => Tokens::Op(Operator::Times),
            ',' => Tokens::Comma,
            // All operator values
            '=' => Tokens::Op(Operator::Eq),
            '&' => Tokens::Op(Operator::And),
            '>' => Tokens::Op(Operator::More),
            '<' => Tokens::Op(Operator::Less),
            // If none of the above tokens match it will a InvalidToken;
            value => Tokens::InvalidToken(TokenErrorMessages::TokenInvalid(value.to_string())),
        }
    }
}

impl ToString for Keywords {
    fn to_string(&self) -> String {
        match self {
            Keywords::If => "if".into(),
            Keywords::Else => "else".into(),
            Keywords::Fn => "fn".into(),
            Keywords::Let => "let".into(),
            Keywords::U8 => "u8".into(),
            Keywords::I8 => "i8".into(),
            Keywords::I32 => "i32".into(),
            Keywords::F32 => "f32".into(),
            Keywords::String => "string".into(),
            Keywords::Char => "char".into(),
            Keywords::Array => "array".into(),
            Keywords::For => "for".into(),
            Keywords::While => "while".into(),
            Keywords::Pub => "pub".into(),
            Keywords::Enum => "enum".into(),
            Keywords::Struct => "struct".into(),
            Keywords::Void => "void".into(),
            Keywords::Return => "return".into(),
        }
    }
}
