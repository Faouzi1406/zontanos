//! The main parser for the language Zontanos
//! Converts the tokens into a Abstract Syntax Tree
#![allow(dead_code)]

pub mod errors;

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

    /// Expects the next token to be a ident, returns the value of the identifier, returns a error
    /// if it is not a identifier
    pub fn parse_next_ident_expr(&mut self) -> ParseResult<Ident> {
        let Some(ident) = self.next() else {return Err(self.expected_ident())};
        if ident.token_type != Tokens::Identifier {
            return Err(self.expected_ident());
        };
        Ok(Ident { name: ident.value })
    }

    /// Parses any generic expr inbetween a <T...>; It does so recursively, meaning any generic can
    /// have more generics;
    ///
    /// It takes in a mutuable base type (the first type/non generic type), it then mutuates thise base types to have the correct
    /// genrecics;
    ///
    /// **It expects to already be in the generics : '<' HERE; so you shouldn't ever walk back
    /// when calling upon ```self.parse_generics_expr()```**
    ///
    /// # Example
    ///
    /// ``array<array<i32>>`` // Can contain a array which contains a array which contains i32/numbers
    pub fn parse_generics_expr(&mut self, base_type: &mut Type) -> ParseResult<()> {
        assert!(self.assert_prev_token().token_type == Tokens::Op(Operator::Less));
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

        // If we get here it means we never got a ending '>' therefore we return and error
        Err(self.expected_end_expr("generics", ">"))
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

    /// If the array contains no parse errors it will return a [`TypeValues::Array`];
    ///
    /// It expects to already be one before the open bracket of the array: >here[not here...; so it
    /// expects the next token to be and open bracket '['
    ///
    /// Consumes all tokens including the ending close bracket ']';
    ///
    /// **It goes one type deep: array<T>; so it will not see array<array<T>>, altough if we start
    /// introducing more complex types, this could be a posibility**
    pub fn parse_array(&mut self, base_type: Type) -> ParseResult<Vec<TypeValues>> {
        let mut array_items: Vec<TypeValues> = Vec::new();
        let Some(inner_array_type)  = base_type.generics.get(0) else {
            return Err(self.expected_array_generic());
        };

        if !self.consume_if_next(Tokens::OpenBracket) {
            return Err("Parse array should only get called if the next token is a open bracket.".to_string());
        }

        let mut curr = TypeValues::None;
        while let Some(array_value) = self.next() {
            match array_value.token_type {
                Tokens::Number | Tokens::FloatNumber | Tokens::String | Tokens::Char => {
                    if curr != TypeValues::None {
                        return Err(self.expected_value_seprator());
                    }
                    let value = inner_array_type
                        .r#type
                        .type_value_convert(&array_value.value)?;
                    curr = value;
                }
                Tokens::Identifier => curr = TypeValues::Identifier(array_value.value),
                Tokens::Comma => {
                    if curr == TypeValues::None {
                        return Err(self.expected_array_value_comma());
                    }
                    array_items.push(curr.clone());
                    curr = TypeValues::None
                }
                Tokens::CloseBracket => {
                    if curr != TypeValues::None {
                        array_items.push(curr.clone())
                    }
                    return Ok(array_items);
                }
                Tokens::OpenBracket => {
                    return 
                    Err(format!("Altough supporting arrays in arrays is planned it is currently not supported, found ']' in array on line {}", array_value.line));
                }
                invalid_token => {
                    return Err(
                        self.invalid_expected_type("array value", &format!("{invalid_token:#?}"))
                    )
                }
            }
        }

        Err(self.expected_end_expr("array", "]"))
    }

    /// Expects the next token to be value
    /// Parses until a end to the value is found depending on the first token;
    /// For example a FunctionCall value will get parsed until the end of the function call ')'
    pub fn parse_value_expr(&mut self, base_type: Type) -> ParseResult<Value> {
        let mut value = Value {
            r#type: TypeValues::None,
        };
        let expected_type = base_type.r#type.clone();
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
            Tokens::OpenBracket => {
                self.walk_back(1);
                value.r#type = TypeValues::Array(self.parse_array(base_type)?);
                return Ok(value);
            }
            Tokens::Identifier => {
                value.r#type = expected_type.type_value_convert(&value_expr.value)?;
                return Ok(value);
            }
            _ => return Err(self.invalid_token_in_expr("value", "value")),
        };
    }

    /// Parses any valid sequence of a variable expression,
    /// consider: let hello: string = "some" // valid
    /// consider: let whut = "some" // invalid no type was given // altought I do want to add type
    /// inference
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
            let variable_value = self.parse_value_expr(var_type)?;
            node.right = Some(Box::from(Node::new(
                NodeTypes::Value(variable_value),
                prev_token.line,
            )));
            Ok(node)
        } else {
            Err(self.expected_assign_token())
        }
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
