//! The main parser for the language Zontanos
//! Converts the tokens into a Abstract Syntax Tree
#![allow(dead_code)]

use crate::zon_parser::lexer::{Keywords, Token, Tokens};

use super::ast::{Ast, Type, Variable};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

type ParseResult<T> = Result<T, String>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> ParseResult<Ast> {
        while let Some(token) = self.next() {
            match token.token_type {
                _ => todo!(),
            }
        }
        todo!()
    }

    pub fn current_token(&mut self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn assert_current_token(&mut self) -> Token {
        let token = self.current_token();
        assert!(token.is_some());
        token.unwrap().clone()
    }

    pub fn consume_if_next(&mut self, next: Tokens) -> bool {
        let Some(token) = self.tokens.get(self.pos + 1) else {return false};
        if token.token_type == next {
            self.next();
            return true;
        }
        false
    }

    pub fn parse_let_expr(&mut self) -> ParseResult<Variable> {
        assert!(self.assert_current_token().token_type == Tokens::Kw(Keywords::Let));
        let Some(ident) = self.next() else {return Err(self.expected_ident())};
        if ident.token_type != Tokens::Identifier {
            return Err(self.expected_ident());
        };
        if !self.consume_if_next(Tokens::Colon) {
            return Err(self.expected_type_seperator());
        }
        todo!()
    }

    pub fn parse_type_expr(&mut self) -> ParseResult<Type> {
        let Some(base_type) = self.next() else {};
        todo!()
    }

    pub fn expected_ident(&mut self) -> String {
        let current = self.assert_current_token();
        let msg = format!(
            "[Parse Error] Expected a variable identifier on line {}",
            current.line
        );
        msg
    }

    pub fn expected_type_seperator(&mut self) -> String {
        let current = self.assert_current_token();
        let msg = format!(
            "[Parse Error] Expected a type seperator ':' on line {} for {}",
            current.line, current.value
        );
        msg
    }
}

impl Iterator for Parser {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let pos = self.tokens.get(self.pos)?;
        self.pos += 1;
        return Some(pos.clone());
    }
}
