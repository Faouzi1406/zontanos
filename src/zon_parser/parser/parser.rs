//! zon_parser/parser.rs
//!
//! This is the main parser of the zontanos;
//! It will be responsible for turning the tokens into the Ast.
//! It should detect whenever there is a invalid set of tokens.

use std::str::FromStr;

use crate::{
    ast::{
        function::Function,
        types::{MarkerTypes, VarTypes},
        variable::{VarData, Variable},
        Ast, AstNodeType,
    },
    zon_parser::{lexer::Operator, parser::parse_errors::ParseErrors},
};

use crate::zon_parser::lexer::{Keywords, Token, Tokens};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current_position: usize,
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.tokens.get(self.current_position)?;
        self.current_position += 1;
        Some(curr.clone())
    }
}

impl Parser {
    /// Returns the previous from the current position
    fn get_prev_token(&self) -> Option<&Token> {
        if self.current_position == 0 {
            return None;
        }
        self.tokens.get(self.current_position - 1)
    }

    /// Returns the next token from the current position without moving the current position
    fn peak_next_token(&self) -> Option<&Token> {
        if self.current_position + 1 == self.tokens.len() {
            return None;
        }
        self.tokens.get(self.current_position + 1)
    }
}

pub trait Parse {
    fn new(tokens: Vec<Token>) -> Self;
    fn parse(&mut self) -> Result<Ast, Vec<String>>;
    fn tokens_until_mut(&mut self, tokens: Tokens) -> Option<Vec<Token>>;
    fn tokens_until(&self, until_token: Tokens) -> Option<Vec<Token>>;
}

pub trait ParseTokens {
    fn parse_var_assignment(&mut self) -> Result<Variable, String>;
    fn parse_type_assignment(&mut self, line: usize) -> Result<VarTypes, String>;
    fn parse_function(&mut self) -> Result<Function, String>;
    fn parse_array(&mut self, array_type: MarkerTypes) -> Result<VarTypes, String>;
}

impl ParseTokens for Parser {
    fn parse_type_assignment(&mut self, line: usize) -> Result<VarTypes, String> {
        let Some(var_type) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };

        let Some(assign) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };
        match assign.token_type {
            // Equals we expect it to be a value assignment, so we continue
            Tokens::Op(Operator::Eq) => {}
            // We would consider thi to be and array of which the value after the colon is a
            // type
            Tokens::Colon => {
                let Some(arr_type) = self.next() else {
                    return Err(ParseErrors::ExpectedNext(line).to_string());
                };
                return Ok(self.parse_array(MarkerTypes::from_str(&arr_type.value)?)?);
            }
            // Any other token type would be considered as a wrong token
            token_type => {
                return Err(
                    ParseErrors::WrongToken(Tokens::Op(Operator::Eq), token_type).to_string(),
                )
            }
        }

        let Some(value) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };
        match value.token_type {
            Tokens::Number => match var_type.token_type {
                Tokens::Kw(Keywords::I32) => {
                    let Ok(value_type) = VarTypes::from_str(&value.value, "i32", line) else {
                        return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                    };
                    return Ok(value_type);
                }
                Tokens::Kw(Keywords::U8) => {
                    let Ok(value_type) = VarTypes::from_str(&value.value, "u8", line) else {
                        return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                    };
                    return Ok(value_type);
                }
                Tokens::Kw(Keywords::I8) => {
                    let Ok(value_type) = VarTypes::from_str(&value.value, "u8", line) else {
                        return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                    };
                    return Ok(value_type);
                }
                _ => return Err(ParseErrors::ExpectedType(line).to_string()),
            },
            Tokens::FloatNumber => {
                if var_type.token_type != Tokens::Kw(Keywords::F32) {
                    return Err(ParseErrors::ExpectedType(line).to_string());
                }
                let Ok(value_type) = VarTypes::from_str(&value.value, "f32", line) else {
                    return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                };
                return Ok(value_type);
            }
            Tokens::String => {
                if var_type.token_type != Tokens::Kw(Keywords::String) {
                    return Err(ParseErrors::ExpectedType(line).to_string());
                }
                let Ok(value_type) = VarTypes::from_str(&value.value, "string", line) else {
                    return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                };
                return Ok(value_type);
            }
            Tokens::Char => {
                if var_type.token_type != Tokens::Kw(Keywords::Char) {
                    return Err(ParseErrors::ExpectedType(line).to_string());
                }
                let Ok(value_type) = VarTypes::from_str(&value.value, "char", line) else {
                            return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                };
                return Ok(value_type);
            }
            _ => return Err(ParseErrors::ExpectedType(line).to_string()),
        }
    }

    fn parse_var_assignment(&mut self) -> Result<Variable, String> {
        let Some(prev_token) = self.get_prev_token() else {
            return Err(ParseErrors::NoPrevToken.to_string());
        };
        let prev_token = prev_token.clone();
        if prev_token.token_type != Tokens::Kw(Keywords::Let) {
            return Err(
                ParseErrors::WrongToken(Tokens::Kw(Keywords::Let), prev_token.token_type)
                    .to_string(),
            );
        }

        let mut variable = Variable::default();
        let Some(var_name) = self.next() else {
            return Err(ParseErrors::ExpectedNext(prev_token.line).to_string());
        };
        if var_name.token_type != Tokens::Identifier {
            return Err(
                ParseErrors::WrongToken(Tokens::Identifier, var_name.token_type).to_string(),
            );
        }

        let _ = variable.set_name(var_name.value, Some(var_name.line));

        let Some(equal_sign) = self.next() else {
            return Err(ParseErrors::ExpectedNext(var_name.line).to_string());
        };

        match equal_sign.token_type {
            // Todo: Type inference
            Tokens::Op(Operator::Eq) => {
                todo!("Type inference");
            }
            // This means the token after colon is the Type expected
            //
            // Example
            //
            // let value: -> string <- = "wow"
            Tokens::Colon => {
                let value = self.parse_type_assignment(equal_sign.line)?;
                let _ = variable.set_type(value, Some(equal_sign.line));
            }
            token => {
                return Err(ParseErrors::WrongToken(Tokens::Op(Operator::Eq), token).to_string())
            }
        }

        return Ok(variable);
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        todo!()
    }

    fn parse_array(&mut self, array_type: MarkerTypes) -> Result<VarTypes, String> {
        let mut array = Vec::new();
        let Some(prev_token) = self.get_prev_token() else {
            return Err("[ParseArray call]: Expected the previous token to be some but it was none.".to_string());
        };
        let prev_token = prev_token.clone();

        let Some(assign) = self.next() else {
            return Err(ParseErrors::ExpectedNext(prev_token.line).to_string());
        };
        if assign.token_type != Tokens::Op(Operator::Eq) {
            return Err(
                ParseErrors::WrongToken(Tokens::Op(Operator::Eq), assign.token_type).to_string(),
            );
        }

        let Some(_expect_open_bracket) = self.next() else {
            return Err(ParseErrors::ExpectedNext(prev_token.line).to_string());
        };
        if _expect_open_bracket.token_type != Tokens::OpenBracket {
            return Err(
                ParseErrors::WrongToken(Tokens::OpenBracket, assign.token_type).to_string(),
            );
        }

        while let Some(token) = self.next() {
            // End of the array
            if token.token_type == Tokens::CloseBracket {
                break;
            };

            match token.token_type {
                Tokens::String => {}
                Tokens::Char => {}
                Tokens::Number => {}
                Tokens::FloatNumber => {}
                _ => return Err(ParseErrors::ExpectedType(prev_token.line).to_string()),
            }
            // We match the expected type against the value of the received token.
            match array_type {
                MarkerTypes::U8 => {
                    let Ok(token) = VarTypes::from_str(&token.value, "u8", prev_token.line) else {
                        return Err(ParseErrors::ExpectedType(prev_token.line).to_string());
                    };
                    array.push(token);
                }
                MarkerTypes::I8 => {
                    let Ok(token) = VarTypes::from_str(&token.value, "i8", prev_token.line) else {
                        return Err(ParseErrors::ExpectedType(prev_token.line).to_string());
                    };
                    array.push(token);
                }
                MarkerTypes::I32 => {
                    let Ok(token) = VarTypes::from_str(&token.value, "i32", prev_token.line) else {
                        return Err(ParseErrors::ExpectedType(prev_token.line).to_string());
                    };
                    array.push(token);
                }
                MarkerTypes::F32 => {
                    let Ok(token) = VarTypes::from_str(&token.value, "f32", prev_token.line) else {
                        return Err(ParseErrors::ExpectedType(prev_token.line).to_string());
                    };
                    array.push(token);
                }
                MarkerTypes::Char => {
                    let Ok(token) = VarTypes::from_str(&token.value, "char", prev_token.line) else {
                        return Err(ParseErrors::ExpectedType(prev_token.line).to_string());
                    };
                    array.push(token);
                }
                MarkerTypes::Identifier => {
                    // Todo: Handle identifiers in arrays
                }
                MarkerTypes::String => {
                    let Ok(token) = VarTypes::from_str(&token.value, "string", prev_token.line) else {
                        return Err(ParseErrors::ExpectedType(prev_token.line).to_string());
                    };
                    array.push(token);
                }
            }
        }
        println!("{array:#?}");
        return Ok(VarTypes::Array { array, array_type });
    }
}

impl Parse for Parser {
    /// Create a new instance of a parser
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current_position: 0,
        }
    }

    /// Parse the tokens into a Ast
    ///
    /// Returns a Ok(Ast) or the Err(Vec<String>) containing all the errors it was able to find
    fn parse(&mut self) -> Result<Ast, Vec<String>> {
        let mut errors = Vec::new();
        let mut ast = Ast::new();
        while let Some(token) = self.next() {
            match token.token_type {
                // We ignore comments
                Tokens::Comment => continue,
                Tokens::Kw(Keywords::Let) => {
                    let assign = self.parse_var_assignment();
                    if let Ok(assign) = assign {
                        ast.insert_node(AstNodeType::Variable(assign));
                        continue;
                    }
                    errors.push(assign.err().unwrap())
                }
                //Tokens:: Parse tokens
                _cant_continue => return Err(errors),
            }
        }
        println!("{:#?}", errors);
        Ok(ast)
    }

    /// Gets all the tokens up until a token; Returns None if the token was not in the array
    ///
    /// Does advance the current_position
    fn tokens_until_mut(&mut self, until_token: Tokens) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        for token in self.by_ref() {
            if token.token_type == until_token {
                return Some(tokens);
            }
            tokens.push(token);
        }
        None
    }

    /// Gets all the tokens up until a token; Returns None if the token was not in the array    
    ///
    /// Does not advance the current position
    fn tokens_until(&self, until_token: Tokens) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        for token in self.tokens.iter() {
            if token.token_type == until_token {
                return Some(tokens);
            }
            tokens.push(token.clone());
        }
        None
    }
}
