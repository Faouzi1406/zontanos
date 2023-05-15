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
            "+" => Tokens::Plus,
            "-" => Tokens::Min,
            "*" => Tokens::Times,
            "," => Tokens::Comma,
            // All operator values
            "=" => Tokens::Op(Operator::Eq),
            "&" => Tokens::Op(Operator::And),
            ">" => Tokens::Op(Operator::More),
            "<" => Tokens::Op(Operator::Less),
            "==" => Tokens::Op(Operator::EqEq),
            "!=" => Tokens::Op(Operator::Nq),
            ">=" => Tokens::Op(Operator::MoreEq),
            "<=" => Tokens::Op(Operator::LessEq),
            "&&" => Tokens::Op(Operator::And),
            // All Keywords
            "let" => Tokens::Kw(Keywords::Let),
            "fn" => Tokens::Kw(Keywords::Fn),
            "for" => Tokens::Kw(Keywords::For),
            "if" => Tokens::Kw(Keywords::If),
            "else" => Tokens::Kw(Keywords::Else),
            "struct" => Tokens::Kw(Keywords::Struct),
            "enum" => Tokens::Kw(Keywords::Enum),
            "pub" => Tokens::Kw(Keywords::Pub),
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
            ';' => Tokens::SemiColon,
            '+' => Tokens::Plus,
            '-' => Tokens::Min,
            '*' => Tokens::Times,
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
