//! The main parser for the language Zontanos
//! Converts the tokens into a Abstract Syntax Tree
#![allow(dead_code)]

pub mod errors;
pub mod lep;
pub mod math;

use super::ast::{
    Assignment, Ast, FunctionCall, Ident, Node, Paramater, Type, Types, Value, Variable,
};
use crate::{
    parser_v2::ast::{Function, NodeTypes, TypeValues},
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

    pub fn peak(&mut self) -> Option<&Token> {
        return self.tokens.get(self.pos + 1);
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
                Tokens::Kw(Keywords::Fn) => {
                    let function = self.parse_fn_expr()?;
                    ast.body.push(function);
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
            is_array: false,
            is_pointer: false,
            size: 0,
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
                Tokens::OpenBracket => {
                    generic_type.is_array = true;
                    if !self.consume_if_next(Tokens::Number) {
                        return Err(self.expected_array_size());
                    }
                    let size = self.assert_prev_token();
                    generic_type.size = size.value.parse().unwrap();
                    if !self.consume_if_next(Tokens::CloseBracket) {
                        return Err(self.expected_end_expr("array type", "]"));
                    }
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

    /// Expects the next stream of tokens to be a type
    /// Parses up intil the '>' en of generics, or end of single type
    pub fn parse_type_expr(&mut self) -> ParseResult<Type> {
        let Some(base_type) = self.next() else { return Err(self.expected_type()) };
        let Tokens::Kw(_) = base_type.token_type else { return Err(self.expected_type()) };
        let mut base_type = Type {
            r#type: Types::from(base_type.value.as_str()),
            generics: Vec::new(),
            is_pointer: false,
            is_array: false,
            size: 0,
        };

        if self.consume_if_next(Tokens::OpenBracket) {
            base_type.is_array = true;
            if self.consume_if_next(Tokens::Number) {
                let value = self.assert_prev_token();
                base_type.size = value.value.parse().unwrap();

                if !self.consume_if_next(Tokens::CloseBracket) {
                    return Err(self.expected_end_expr("array type", "]"));
                }
            } else {
                return Err(self.expected_array_size());
            }
        }

        if self.consume_if_next(Tokens::Op(Operator::Less)) {
            self.parse_generics_expr(&mut base_type)?;
            return Ok(base_type);
        }

        if self.consume_if_next(Tokens::Pointer) {
            base_type.is_pointer = true;
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
    pub fn parse_array(&mut self, base_type: &Type) -> ParseResult<Vec<TypeValues>> {
        let mut array_items: Vec<TypeValues> = Vec::new();
        if !base_type.is_array {
            return Err(self.expected_array_generic());
        }

        if !self.consume_if_next(Tokens::OpenBracket) {
            return Err(
                "Parse array should only get called if the next token is a open bracket."
                    .to_string(),
            );
        }

        let mut curr = TypeValues::None;
        while let Some(array_value) = self.next() {
            match array_value.token_type {
                Tokens::Number | Tokens::FloatNumber | Tokens::String | Tokens::Char => {
                    if curr != TypeValues::None {
                        return Err(self.expected_value_seprator());
                    }
                    let value = base_type.r#type.type_value_convert(&array_value.value)?;
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
                    return Err(self.not_supported_array_in_array());
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
    pub fn parse_value_expr(&mut self, base_type: &Type) -> ParseResult<Node> {
        let mut value = Value {
            value: TypeValues::None,
            is_ptr: false,
        };

        let expected_type = base_type.r#type.clone();

        if self.consume_if_next(Tokens::Pointer) {
            value.is_ptr = true;
        }

        let Some(value_expr) = self.next() else {
            return Err(self.invalid_expected_type("value", "none"));
        };

        match value_expr.token_type {
            Tokens::Number | Tokens::Char | Tokens::FloatNumber | Tokens::String => {
                value.value = expected_type.type_value_convert(&value_expr.value)?;
                return Ok(Node::new(NodeTypes::Value(value), value_expr.line));
            }
            Tokens::OpenBracket => {
                self.walk_back(1);
                let arr = self.parse_array(base_type)?;
                value.value = TypeValues::Array(arr);
                return Ok(Node::new(NodeTypes::Value(value), value_expr.line));
            }
            Tokens::Identifier => {
                if self.consume_if_next(Tokens::OpenBrace) {
                    self.walk_back(2);
                    let (function_call, arguments) = self.parse_fn_call_expr()?;
                    let function_call = Node::fn_call(function_call, arguments, value_expr.line);
                    return Ok(function_call);
                }
                value.value = TypeValues::Identifier(value_expr.value);
                return Ok(Node::new(NodeTypes::Value(value), value_expr.line));
            }
            Tokens::Kw(Keywords::Void) => {
                return Ok(Node::new(
                    NodeTypes::Value(Value {
                        value: TypeValues::None,
                        is_ptr: false,
                    }),
                    value_expr.line,
                ));
            }
            _ => return Err(self.invalid_token_in_expr("value", "value")),
        };
    }

    /// Parses any valid sequence of a variable expression,
    /// consider: let hello: string = "some" // valid
    /// consider: let whut = "some" // invalid no type was given // altought I do want to add type
    /// inference
    pub fn parse_let_expr(&mut self) -> ParseResult<Node> {
        let Some(next_token) = self.next() else {
            return Err(self.invalid_expected_type("variable", "none"))
        };
        assert!(next_token.token_type == Tokens::Kw(Keywords::Let));

        let ident = self.parse_next_ident_expr()?;
        if !self.consume_if_next(Tokens::Colon) {
            return Err(self.expected_type_seperator());
        }

        let mut var_type = self.parse_type_expr()?;

        if self.consume_if_next(Tokens::Op(Operator::Eq)) {
            let mut node = Node::variable(
                Variable {
                    ident,
                    var_type: var_type.clone(),
                },
                Operator::Eq,
                next_token.line,
            );
            let variable_value = self.parse_value_expr(&mut var_type)?;
            node.right = Some(Box::new(variable_value));
            Ok(node)
        } else {
            Err(self.expected_assign_token())
        }
    }

    /// Expects to be before the openbrace `POS_HERE-next->(` when getting called upon;
    ///
    /// # Example of paramaters
    /// `(id_0: string, id1: array<i32>)`
    pub fn parse_params(&mut self) -> ParseResult<Vec<Paramater>> {
        if !self.consume_if_next(Tokens::OpenBrace) {
            return Err(self.expected_params_openbrace());
        };

        let mut params = Vec::new();
        if self.consume_if_next(Tokens::CloseBrace) {
            return Ok(params);
        }

        while let Some(_next_param) = self.next() {
            self.walk_back(1);

            let ident = self.parse_next_ident_expr()?;
            if !self.consume_if_next(Tokens::Colon) {
                return Err(self.expected_type_seperator());
            }

            let type_param = self.parse_type_expr()?;
            params.push(Paramater {
                r#type: type_param,
                ident,
            });

            if self.consume_if_next(Tokens::CloseBrace) {
                return Ok(params);
            }

            if !self.consume_if_next(Tokens::Comma) {
                return Err(self.expected_type_seperator());
            }
        }

        return Err(self.expected_end_expr("paramaters", ")"));
    }

    fn parse_not_know_type_value(&mut self) -> ParseResult<Value> {
        let mut value_holder = Value {
            value: TypeValues::None,
            is_ptr: false,
        };
        if self.consume_if_next(Tokens::Pointer) {
            value_holder.is_ptr = true;
        }

        let Some(value) = self.next() else {
            //todo fix tis!
            return Err(self.expected_value_seprator());
        };

        // we assume defaults here, consider float to be f32, number to be i32, etc.
        match value.token_type {
            Tokens::String => {
                let none_type = Types::String;
                let value = none_type.type_value_convert(&value.value)?;
                value_holder.value = value;
                Ok(value_holder)
            }
            Tokens::Char => {
                let none_type = Types::Char;
                let value = none_type.type_value_convert(&value.value)?;
                value_holder.value = value;
                Ok(value_holder)
            }
            Tokens::Number => {
                let none_type = Types::I32;
                let value = none_type.type_value_convert(&value.value)?;
                value_holder.value = value;
                Ok(value_holder)
            }
            Tokens::FloatNumber => {
                let none_type = Types::F32;
                let value = none_type.type_value_convert(&value.value)?;
                value_holder.value = value;
                Ok(value_holder)
            }
            Tokens::Identifier => {
                if self.consume_if_next(Tokens::OpenBrace) {
                    self.walk_back(2);
                    let (call, args) = self.parse_fn_call_expr()?;
                    let NodeTypes::Arguments(args) = args else { unreachable!("ERROR: EXPECTED ARGUMENTS FROM PARSE FN CALL") };
                    value_holder.value = TypeValues::FunctionCall(call, args);
                    return Ok(value_holder.into());
                }
                let none_type = Types::Ident;
                let value = none_type.type_value_convert(&value.value)?;
                value_holder.value = value;
                Ok(value_holder)
            }
            _ => Err(self.invalid_token_in_expr("value", "value")),
        }
    }

    pub fn parse_args_expr(&mut self) -> ParseResult<Vec<Value>> {
        // Todo: change this :|
        assert_eq!(self.next().unwrap().token_type, Tokens::OpenBrace);

        let mut values = Vec::new();

        if self.consume_if_next(Tokens::CloseBrace) {
            return Ok(values);
        }

        while let Some(_) = self.next() {
            self.walk_back(1);
            let value = self.parse_not_know_type_value()?;
            values.push(value);

            if self.consume_if_next(Tokens::Comma) {
                continue;
            }
            if self.consume_if_next(Tokens::CloseBrace) {
                return Ok(values);
            }

            return Err(self.expected_end_expr("argument", ")"));
        }

        Err(self.expected_end_expr("argument", ")"))
    }

    pub fn parse_reassignment_expr(&mut self) -> ParseResult<Node> {
        let assigns_to = self.parse_next_ident_expr()?;
        if let Some(token) = self.next() {
            if let Tokens::Op(op) = token.token_type {
                let value = self.parse_not_know_type_value()?;
                let assignment = Assignment { assigns_to };
                let node = Node {
                    node_type: NodeTypes::Assignment(assignment),
                    right: Some(Box::new(Node::new(NodeTypes::Value(value), token.line))),
                    left: Some(Box::new(Node::new(NodeTypes::Operator(op), token.line))),
                    line: token.line,
                };
                return Ok(node);
            }
        }
        Err(self.expected_assign_token())
    }

    /// Returns the block node and the end line
    pub fn parse_block_expr(&mut self, type_expected: &Type) -> ParseResult<(Vec<Node>, usize)> {
        let Some(currly_open) = self.next() else {
            return Err(self.expected_body_openbracket());
        };
        assert_eq!(currly_open.token_type, Tokens::OpenCurlyBracket);
        let mut body = Vec::new();

        while let Some(body_token) = self.next() {
            match body_token.token_type {
                Tokens::Kw(Keywords::Let) => {
                    self.walk_back(1);
                    let parse_let = self.parse_let_expr()?;
                    body.push(parse_let)
                }
                Tokens::Kw(Keywords::Fn) => {
                    let parse_fn = self.parse_fn_expr()?;
                    body.push(parse_fn);
                }
                Tokens::OpenBracket => {
                    let (block, line) = self.parse_block_expr(type_expected)?;
                    body.push(Node::new(NodeTypes::Block(block), line))
                }
                Tokens::Identifier => {
                    // Handle re-assignments
                    self.walk_back(1);
                    let (func_call, arguments) = self.parse_fn_call_expr()?;
                    let func_call_node = Node::fn_call(func_call, arguments, body_token.line);
                    body.push(func_call_node);
                }
                Tokens::CloseCurlyBracket => {
                    return Ok((body, body_token.line));
                }
                Tokens::Kw(Keywords::Return) => {
                    let return_node = self.parse_return_value(type_expected)?;
                    body.push(return_node);
                }
                _ => {
                    return Err(format!(
                        "Found a token that is invalid in the body of a token: {:#?} on line {}",
                        body_token.value, body_token.line
                    ))
                }
            }
        }

        Err(self.expected_end_expr("body", "}"))
    }

    pub fn parse_return_value(&mut self, type_expected: &Type) -> ParseResult<Node> {
        let value = self.parse_value_expr(&type_expected)?;
        let mut node = Node::new(NodeTypes::Return, value.line);
        node.right = Some(Box::new(value));
        Ok(node)
    }

    /// Returns the function call it self, and it's arguments
    pub fn parse_fn_call_expr(&mut self) -> ParseResult<(FunctionCall, NodeTypes)> {
        let ident = self.parse_next_ident_expr()?;
        let arguments = self.parse_args_expr()?;
        Ok((
            FunctionCall { calls_to: ident },
            NodeTypes::Arguments(arguments),
        ))
    }

    /// Parses any valid function statement, starting from the identifier up until the ending close
    /// bracket;
    ///
    /// # Example
    ///
    /// fn `->starts here` some(..) {
    ///     body
    /// }
    pub fn parse_fn_expr(&mut self) -> ParseResult<Node> {
        let ident = self.parse_next_ident_expr()?;
        let paramaters = self.parse_params()?;
        let mut returns = self.parse_type_expr()?;
        let (body, line) = self.parse_block_expr(&mut returns)?;
        let function = Function {
            returns,
            ident,
            body,
            paramaters,
        };
        Ok(Node::new(NodeTypes::Function(function), line))
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
