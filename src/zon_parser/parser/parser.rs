//! zon_parser/parser.rs
//!
//! This is the main parser of the zontanos;
//! It will be responsible for turning the tokens into the Ast.
//! It should detect whenever there is a invalid set of tokens.

use crate::zon_parser::parser::parse_functions::{FunctionCalls, FunctionParser};
use crate::{
    ast::{
        block::Block,
        types::{MarkerTypes, VarTypes},
        variable::{VarData, Variable},
        Ast, Expr,
    },
    zon_parser::{lexer::Operator, parser::parse_errors::ParseErrors},
};
use std::println;
use std::str::FromStr;

use crate::zon_parser::lexer::{Keywords, Token, Tokens};

use super::logical_parser::IfElseParser;

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
    pub fn get_prev_token(&self) -> Option<&Token> {
        if self.current_position == 0 {
            return None;
        }
        self.tokens.get(self.current_position - 1)
    }

    /// Advance the current position back by n amount
    pub fn advance_back(&mut self, n: usize) {
        self.current_position -= n;
    }
}

pub trait ParseTokens {
    fn parse_var_assignment(&mut self) -> Result<Variable, String>;
    fn parse_type_assignment(&mut self, line: usize) -> Result<VarTypes, String>;
    fn parse_array(&mut self, array_type: MarkerTypes) -> Result<VarTypes, String>;
    fn parse_block(&mut self, return_type: MarkerTypes) -> Result<Block, String>;
}

impl ParseTokens for Parser {
    fn parse_var_assignment(&mut self) -> Result<Variable, String> {
        let Some(prev_token) = self.get_prev_token() else {
            return Err(ParseErrors::NoPrevToken.to_string());
        };
        let prev_token = prev_token.clone();
        Parser::parse_expect(prev_token.token_type, Tokens::Kw(Keywords::Let))?;

        let mut variable = Variable::default();
        let Some(var_name) = self.next() else {
            return Err(ParseErrors::ExpectedNext(prev_token.line).to_string());
        };
        Parser::parse_expect(var_name.token_type, Tokens::Identifier)?;

        let _ = variable.set_name(var_name.value, Some(var_name.line));
        variable.var_line = prev_token.line;

        let Some(variable_assignment) = self.next() else {
            return Err(ParseErrors::ExpectedNext(var_name.line).to_string());
        };
        match variable_assignment.token_type {
            Tokens::Op(Operator::Eq) => {
                let type_inference_error = format!(
                    "Found a assignment on line {} but didn't get a type, currently type inference isn't supported so pleass consider adding a type.", 
                    variable_assignment.line
                );
                return Err(type_inference_error);
            }
            Tokens::Colon => {
                let value = self.parse_type_assignment(variable_assignment.line)?;
                let _ = variable.set_type(value, Some(variable_assignment.line));
            }
            token => return Err(ParseErrors::WrongToken(Tokens::Colon, token).to_string()),
        }

        return Ok(variable);
    }

    fn parse_type_assignment(&mut self, line: usize) -> Result<VarTypes, String> {
        // The token after colon would be the type
        let Some(var_type) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };

        // The next token either being the assign or the generic value
        let Some(assign) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };

        match assign.token_type {
            Tokens::Op(Operator::Eq) => {}
            // If it is a colon it specifies a type that contains other types, currently that would
            // only be and Array.
            //
            // This will probably change as generics get introduced in to the language.
            Tokens::Colon => {
                let Some(arr_type) = self.next() else {
                    return Err(ParseErrors::ExpectedNext(line).to_string());
                };

                let Some(expect_assign) = self.next() else {
                    return Err(ParseErrors::ExpectedNext(line).to_string());
                };
                Parser::parse_expect(expect_assign.token_type, Tokens::Op(Operator::Eq))?;

                let type_array = MarkerTypes::from_str(&arr_type.value)?;
                return Ok(self.parse_array(type_array)?);
            }
            _ => {
                let no_given_type = format!("Found a variable with a colon one line {line} but the value after the colon was not of a type");
                return Err(no_given_type);
            }
        }

        let Some(value) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };
        match value.token_type {
            Tokens::Number => match var_type.token_type {
                Tokens::Kw(keyword) => {
                    let Ok(value_type) = VarTypes::from_str(&value.value, &keyword.to_string(), line) else {
                        return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                    };
                    return Ok(value_type);
                }
                _ => return Err(ParseErrors::ExpectedType(line).to_string()),
            },
            Tokens::FloatNumber => {
                Parser::parse_expect(var_type.token_type, Tokens::Kw(Keywords::F32))?;
                let Ok(value_type) = VarTypes::from_str(&value.value, "f32", line) else {
                    return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                };
                return Ok(value_type);
            }
            Tokens::String => {
                Parser::parse_expect(var_type.token_type, Tokens::Kw(Keywords::String))?;
                let Ok(value_type) = VarTypes::from_str(&value.value, "string", line) else {
                    return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                };
                return Ok(value_type);
            }
            Tokens::Char => {
                Parser::parse_expect(var_type.token_type, Tokens::Kw(Keywords::Char))?;
                let Ok(value_type) = VarTypes::from_str(&value.value, "char", line) else {
                    return Err(format!("Expected the value to be the same type as the variable but it was not on line {line}"));
                };
                return Ok(value_type);
            }
            Tokens::Identifier => {
                let value = self.parse_value(value, MarkerTypes::from_str(&var_type.value)?)?;
                return Ok(value);
            }
            _ => return Err(ParseErrors::ExpectedType(line).to_string()),
        }
    }

    fn parse_array(&mut self, array_type: MarkerTypes) -> Result<VarTypes, String> {
        let mut array = Vec::new();
        let Some(prev_token) = self.get_prev_token() else {
            return Err("[ParseArray call]: Expected the previous token to be some but it was none.".to_string());
        };
        let prev_token = prev_token.clone();

        let Some(expect_open_bracket) = self.next() else {
            return Err(ParseErrors::ExpectedNext(prev_token.line).to_string());
        };
        Parser::parse_expect(expect_open_bracket.token_type, Tokens::OpenBracket)?;

        while let Some(token) = self.next() {
            if token.token_type == Tokens::CloseBracket {
                break;
            };

            match token.token_type {
                Tokens::String => {}
                Tokens::Char => {}
                Tokens::Number => {}
                Tokens::FloatNumber => {}
                Tokens::Identifier => {
                    let value = self.parse_value(token, array_type.clone())?;
                    array.push(value);
                    continue;
                }
                _ => return Err(ParseErrors::ExpectedType(prev_token.line).to_string()),
            }
            // We match the expected type against the value of the received token.
            match &array_type {
                MarkerTypes::None => {
                    let Ok(token) = VarTypes::from_str(&token.value, "string", prev_token.line) else {
                        return Err(ParseErrors::ExpectedType(prev_token.line).to_string());
                    };
                    array.push(token);
                }
                _ => {
                    if array_type.to_string() != token.token_type.to_string() {
                        return Err(format!(
                            "Expected array value to be of type {} but it was of {} on line {}",
                            array_type.to_string(),
                            token.token_type.to_string(),
                            token.line
                        ));
                    };
                    let Ok(token) = VarTypes::from_str(&token.value, &array_type.to_string(), prev_token.line) else {
                        return Err(ParseErrors::ExpectedType(prev_token.line).to_string());
                    };
                    array.push(token);
                }
            }
        }
        return Ok(VarTypes::Array { array, array_type });
    }

    fn parse_block(&mut self, array_type: MarkerTypes) -> Result<Block, String> {
        let ast_vec = Vec::new();

        let Some(open_block) = self.next() else {
            return Err(ParseErrors::ExpectedNext(0).to_string());
        };
        if open_block.token_type != Tokens::OpenCurlyBracket {
            return Err(
                ParseErrors::InvalidToken(open_block.line, open_block.token_type).to_string(),
            );
        }

        let mut block = Block {
            body: ast_vec,
            line: open_block.line,
        };

        while let Some(token) = self.next() {
            match token.token_type {
                Tokens::Kw(Keywords::Let) => {
                    let var = self.parse_var_assignment()?;
                    block.insert_node(Expr::Variable(var));
                }
                Tokens::Kw(Keywords::If) => {
                    self.advance_back(1);
                    let if_statement = self.parse_if(token.line)?;
                    block.insert_node(if_statement);
                }
                Tokens::Identifier => {
                    let function_call = self.parse_function_call(token.value, token.line)?;
                    block.insert_node(Expr::FunctionCall(function_call))
                }
                Tokens::Kw(Keywords::Fn) => {
                    let func = self.parse_function()?;
                    block.insert_node(Expr::Function(func));
                }
                Tokens::OpenCurlyBracket => {
                    self.advance_back(1);
                    block.insert_node(Expr::Block(self.parse_block(MarkerTypes::None)?));
                }
                Tokens::Kw(Keywords::Return) => {
                    let return_value = self.parse_ret(array_type.clone())?;
                    block.insert_node(Expr::Return(return_value));
                }
                Tokens::CloseCurlyBracket => {
                    return Ok(block);
                }
                token => return Err(ParseErrors::InvalidToken(open_block.line, token).to_string()),
            }
        }

        Err(ParseErrors::NoEnd(open_block.line).to_string())
    }
}

impl Parser {
    /// Create a new instance of a parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current_position: 0,
        }
    }

    /// Parse the tokens into a Ast
    ///
    /// Returns a Ok(Ast) or the Err(Vec<String>) containing all the errors it was able to find
    pub fn parse<'ctx>(&mut self) -> Result<Ast, Vec<String>> {
        let mut errors = Vec::new();
        let mut ast = Ast::new();

        while let Some(token) = self.next() {
            match token.token_type {
                // Ignore comments
                Tokens::Comment => continue,
                Tokens::InvalidToken(_) => continue,
                Tokens::Kw(Keywords::Let) => {
                    let assign = self.parse_var_assignment();
                    if let Ok(assign) = assign {
                        ast.insert_node(Expr::Variable(assign));
                        continue;
                    }
                    errors.push(assign.err().unwrap())
                }
                Tokens::Kw(Keywords::Fn) => {
                    let function = self.parse_function();
                    if let Ok(parsed_function) = function {
                        ast.insert_node(Expr::Function(parsed_function));
                    } else {
                        errors.push(function.err().unwrap());
                        return Err(errors);
                    }
                }
                Tokens::Kw(Keywords::If) => {
                    self.advance_back(1);
                    let if_statement = self.parse_if(token.line);
                    let Ok(if_statement) = if_statement else {
                        errors.push(if_statement.err().unwrap());
                        return Err(errors);
                    };
                    ast.insert_node(if_statement);
                }
                Tokens::OpenCurlyBracket => {
                    self.advance_back(1);
                    let parse_block = self.parse_block(MarkerTypes::None);
                    let Ok(parsed_block) = parse_block else {
                        errors.push(parse_block.err().unwrap());
                        return Err(errors);
                    };
                    ast.insert_node(Expr::Block(parsed_block));
                }
                _cant_continue => {
                    println!("found invalid token: {_cant_continue:#?}");
                    return Err(errors);
                }
            }
        }

        Ok(ast)
    }

    pub fn parse_value(
        &mut self,
        token: Token,
        marker_type: MarkerTypes,
    ) -> Result<VarTypes, String> {
        let line = token.line;
        match token.token_type {
            Tokens::OpenBracket => {
                self.advance_back(1);
                self.parse_array(marker_type)
            },
            Tokens::Identifier => {
                let Some(is_call) = self.next() else {
                    return Err(ParseErrors::ExpectedNext(line).to_string());
                };

                // Consider function call after assignment;
                if let Tokens::OpenBrace = is_call.token_type {
                    return Ok(VarTypes::FunctionCall(
                        self.parse_function_call(token.value, line)?,
                        marker_type,
                    ));
                }

                self.advance_back(1);
                return Ok(VarTypes::Identifier(token.value, MarkerTypes::None));
            }
            Tokens::String | Tokens::Number | Tokens::FloatNumber | Tokens::Char => {
                let Ok(var_type) = VarTypes::from_str(&token.value, &token.token_type.to_string(), token.line) else {
                    return Err(ParseErrors::ExpectedType(token.line).to_string());
                };
                Ok(var_type)
            }
            invalid_token => {
                return Err(ParseErrors::InvalidToken(token.line, invalid_token).to_string())
            }
        }
    }

    pub fn parse_operator(token: Token) -> Result<Operator, String> {
        match token.token_type {
            Tokens::Op(op) => Ok(op),
            invalid_token => Err(ParseErrors::InvalidToken(token.line, invalid_token).to_string()),
        }
    }

    /// Returns the expected token while parsing if the received token wasn't the expected one it
    /// returns a error.
    pub fn parse_expect(token: Tokens, expected: Tokens) -> Result<Tokens, String> {
        if token == expected {
            return Ok(token);
        }
        return Err(format!(
            "Expected token was: {expected:#?} but received token was {token:#?} "
        ));
    }
}
