//! The main parser for the language Zontanos
//! Converts the tokens into a Abstract Syntax Tree
#![allow(dead_code)]

use super::ast::{Ast, Ident, Node, Type, Types, Value, Variable};
use crate::{
    parser_v2::ast::{NodeTypes, TypeValues},
    zon_parser::lexer::{Keywords, Operator, Token, Tokens},
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

type ParseResult<T> = Result<T, String>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn walk_back(&mut self, n: usize) {
        assert!(self.pos > 0);
        self.pos -= n;
    }

    pub fn parse(&mut self) -> ParseResult<Ast> {
        let mut ast = Ast {
            body: Vec::new(),
            r#type: NodeTypes::Program,
        };
        while let Some(token) = self.next() {
            match token.token_type {
                Tokens::Kw(Keywords::Let) => {
                    // We found the let token, but parse_let_expr expects the
                    // next token to be a let token so we walk one back
                    self.walk_back(1);
                    let let_expr = self.parse_let_expr()?;
                    ast.body.push(let_expr);
                }
                kw => todo!("found {kw:#?}"),
            }
        }
        Ok(ast)
    }

    pub fn prev_token(&mut self) -> Option<&Token> {
        self.tokens.get(self.pos - 1)
    }

    pub fn assert_prev_token(&mut self) -> Token {
        let token = self.prev_token();
        assert!(token.is_some());
        token.unwrap().clone()
    }

    pub fn consume_if_next(&mut self, next: Tokens) -> bool {
        let Some(token) = self.tokens.get(self.pos) else {return false};
        if token.token_type == next {
            self.next();
            return true;
        }
        false
    }

    pub fn parse_let_expr(&mut self) -> ParseResult<Node> {
        let Some(prev_token) = self.next() else {
            return Err(self.invalid_expected_type("variable", "none"))
        };
        assert!(prev_token.token_type == Tokens::Kw(Keywords::Let));

        let ident = self.parse_next_ident_expr()?;
        if !self.consume_if_next(Tokens::Colon) {
            return Err(self.expected_type_seperator());
        }

        let var_type = self.parse_type_expr()?;
        let mut node = Node {
            node_type: NodeTypes::Variable(Variable {
                ident,
                var_type: var_type.clone(),
            }),
            left: None,
            right: None,
            line: prev_token.line,
        };

        if self.consume_if_next(Tokens::Op(Operator::Eq)) {
            node.left = Some(Box::from(Node::new(
                NodeTypes::Operator(Operator::Eq),
                prev_token.line,
            )));
            let variable_value = self.parse_value_expr(var_type.r#type)?;
            node.right = Some(Box::from(Node::new(
                NodeTypes::Value(variable_value),
                prev_token.line,
            )));
            Ok(node)
        } else {
            Err(self.expected_assign_token())
        }
    }

    pub fn parse_next_ident_expr(&mut self) -> ParseResult<Ident> {
        let Some(ident) = self.next() else {return Err(self.expected_ident())};
        if ident.token_type != Tokens::Identifier {
            return Err(self.expected_ident());
        };
        Ok(Ident { name: ident.value })
    }

    /// Expects the next token to be value
    /// Parses until a end to the value is found depending on the first token;
    /// For example a FunctionCall value will get parsed until the end of the function call ')'
    pub fn parse_value_expr(&mut self, expected_type: Types) -> ParseResult<Value> {
        let mut value = Value {
            r#type: TypeValues::None,
        };
        let Some(value_expr) = self.next() else {
            return Err(self.invalid_expected_type("value", "none"));
        };
        match value_expr.token_type {
            Tokens::Number => {
                value.r#type = expected_type.type_value_convert(&value_expr.value)?;
                return Ok(value);
            }
            Tokens::FloatNumber => {
                value.r#type = expected_type.type_value_convert(&value_expr.value)?;
                return Ok(value);
            }
            Tokens::String => {
                value.r#type = expected_type.type_value_convert(&value_expr.value)?;
                return Ok(value);
            }
            Tokens::Identifier => {}
            _ => return Err(self.invalid_token_in_expr("value", "value")),
        };
        Ok(value)
    }

    /// Expects the next token to be a ident
    pub fn parse_current_ident_expr(&mut self) -> ParseResult<Ident> {
        let Some(ident) = self.next() else {return Err(self.expected_ident())};
        if ident.token_type != Tokens::Identifier {
            return Err(self.expected_ident());
        };
        Ok(Ident {
            name: ident.value.clone(),
        })
    }

    /// Expects the next stream of tokens to be a type
    /// Parses up intil the '>' en of generics, or end of single type
    pub fn parse_type_expr(&mut self) -> ParseResult<Type> {
        let Some(base_type) = self.next() else { return Err(self.expected_type()) };
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
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a type after ':' on line {}",
            current.line
        );
        msg
    }

    pub fn expected_end_expr(&mut self, to: &str, end: &str) -> String {
        self.pos -= 1;
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a end to {to} '{end}' on line {}",
            current.line
        );
        msg
    }

    pub fn comma_without_type_generic(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a comma after a type <T, T> on line {}",
            current.line
        );
        msg
    }

    pub fn invalid_token_in_expr(&mut self, expr: &str, expected: &str) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Found a invalid token while parsing {expr} on line {}, expected {expected} got {}",
            current.line, current.value
        );
        msg
    }

    pub fn expected_assign_token(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected assignment token on line {}",
            current.line
        );
        msg
    }

    pub fn expected_ident(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a variable identifier on line {}",
            current.line
        );
        msg
    }

    pub fn expected_type_seperator(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a type seperator ':' on line {} for {}",
            current.line, current.value
        );
        msg
    }

    pub fn invalid_expected_type(&mut self, type_expected: &str, value_received: &str) -> String {
        let msg =
            format!("[Parse Error] Expected a type of {type_expected} but got {value_received}",);
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
