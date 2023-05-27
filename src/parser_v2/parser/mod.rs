//! The main parser for the language Zontanos
//! Converts the tokens into a Abstract Syntax Tree
#![allow(dead_code)]

use super::ast::{Ast, Ident, Type, Types, Variable, Node};
use crate::zon_parser::lexer::{Keywords, Operator, Token, Tokens};

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

    pub fn parse_let_expr(&mut self) -> ParseResult<Node> {
        assert!(self.assert_current_token().token_type == Tokens::Kw(Keywords::Let));
        let ident = self.parse_next_ident_expr();
        if !self.consume_if_next(Tokens::Colon) {
            return Err(self.expected_type_seperator());
        }
        let var_type = self.parse_type_expr()?;
        let node = Node {
            //node_type: 
        };
        todo!()
    }

    pub fn parse_next_ident_expr(&mut self) -> ParseResult<Ident> {
        let Some(ident) = self.next() else {return Err(self.expected_ident())};
        if ident.token_type != Tokens::Identifier {
            return Err(self.expected_ident());
        };
        Ok(Ident { name: ident.value })
    }

    pub fn parse_current_ident_expr(&mut self) -> ParseResult<Ident> {
        let Some(ident) = self.current_token() else {return Err(self.expected_ident())};
        if ident.token_type != Tokens::Identifier {
            return Err(self.expected_ident());
        };
        Ok(Ident {
            name: ident.value.clone(),
        })
    }

    pub fn parse_type_expr(&mut self) -> ParseResult<Type> {
        let Some(base_type) = self.current_token() else { return Err(self.expected_type()) };
        let Tokens::Kw(_) = base_type.token_type else {return Err(self.expected_type())};
        let mut base_type = Type {
            r#type: Types::from(base_type.value.as_str()),
            generics: Vec::new(),
        };

        if self.consume_if_next(Tokens::Op(Operator::Less)) {
            self.parse_generics_expr(&mut base_type)?;
            return Ok(base_type);
        }
        return Ok(base_type);
    }

    pub fn parse_generics_expr(&mut self, base_type: &mut Type) -> ParseResult<()> {
        let mut generic_type = Type {
            r#type: Types::UnknownType("".into()),
            generics: Vec::new(),
        };

        while let Some(generic) = self.next() {
            match generic.token_type {
                Tokens::Kw(_) => {
                    if generic_type.r#type != Types::UnknownType("".into()) {
                        return Err(self.expected_type_seperator());
                    }
                    generic_type.r#type = Types::from(generic.value.as_str());
                }
                Tokens::Comma => {
                    if generic_type.r#type == Types::UnknownType("".into()) {
                        continue;
                    }
                    base_type.generics.push(generic_type.clone());
                    generic_type.r#type = Types::UnknownType("".into())
                }
                Tokens::Op(Operator::Less) => {
                    self.parse_generics_expr(&mut generic_type)?;
                    base_type.generics.push(generic_type.clone());

                    generic_type.r#type = Types::UnknownType("".into());
                    generic_type.generics.clear();
                }
                Tokens::Op(Operator::More) => {
                    if generic_type.r#type != Types::UnknownType("".into()) {
                        base_type.generics.push(generic_type.clone());
                    }
                    return Ok(());
                }
                _ => return Err(self.invalid_token_in_expr("generics", "type")),
            }
        }

        Err(self.expected_end_expr("generics", ">"))
    }

    pub fn expected_type(&mut self) -> String {
        let current = self.assert_current_token();
        let msg = format!(
            "[Parse Error] Expected a type after ':' on line {}",
            current.line
        );
        msg
    }

    pub fn expected_end_expr(&mut self, to: &str, end: &str) -> String {
        self.pos -= 1;
        let current = self.assert_current_token();
        let msg = format!(
            "[Parse Error] Expected a end to {to} '{end}' on line {}",
            current.line
        );
        msg
    }

    pub fn comma_without_type_generic(&mut self) -> String {
        let current = self.assert_current_token();
        let msg = format!(
            "[Parse Error] Expected a comma after a type <T, T> on line {}",
            current.line
        );
        msg
    }

    pub fn invalid_token_in_expr(&mut self, expr: &str, expected: &str) -> String {
        let current = self.assert_current_token();
        let msg = format!(
            "[Parse Error] Found a invalid token while parsing {expr} on line {}, expected {expected} got {}",
            current.line, current.value
        );
        msg
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
        self.pos += 1;
        let pos = self.tokens.get(self.pos)?;
        return Some(pos.clone());
    }
}
