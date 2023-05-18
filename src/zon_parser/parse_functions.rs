use std::format;

use super::{
    lexer::{Keywords, Tokens},
    parser::{
        parse_errors::ParseErrors,
        parser::{ParseTokens, Parser},
    },
};
use crate::ast::{
    function::{Function, Paramater},
    types::MarkerTypes,
};

pub trait FunctionParser {
    /// Parses the paramaters of a function
    ///
    /// # Example;
    ///
    /// (i32 x, i32 y, i32 z)
    ///
    /// Parse paramater will return a error if it was called while the previous token wasn't a
    /// openbrace ')'
    fn parse_params(&mut self) -> Result<Vec<Paramater>, String>;
    /// Parses the return type of a function
    ///
    /// # Example
    ///
    ///  i32 // returns the MarkerTypes::I32
    ///  string // returns the MarkerTypes::String
    ///  char // returns the MarkerTypes::String
    fn parse_function_return_type(&mut self, line: usize) -> Result<MarkerTypes, String>;
    /// Is expected to be called after a Fn token is found
    /// Parses a valid function
    ///
    /// # Example
    ///
    /// fn main() i32 {
    ///     return 1;
    /// }
    fn parse_function(&mut self) -> Result<Function, String>;
}

impl FunctionParser for Parser {
    fn parse_params(&mut self) -> Result<Vec<Paramater>, String> {
        let mut paramaters = Vec::new();
        let Some(prev_token) = self.get_prev_token() else {
            return Err("Expected the previous token to be some".to_string());
        };
        let prev_token = prev_token.clone();

        let Some(open_brace) = self.next() else {
            return Err(ParseErrors::ExpectedNext(prev_token.line).to_string());
        };
        if open_brace.token_type != Tokens::OpenBrace {
            return Err(
                ParseErrors::InvalidToken(prev_token.line, open_brace.token_type).to_string(),
            );
        }
        let line = open_brace.line;

        let mut curr_paramater = Paramater::new("", MarkerTypes::None);
        while let Some(tokens) = self.next() {
            match tokens.token_type {
                Tokens::Kw(kw) => match kw {
                    Keywords::String
                    | Keywords::U8
                    | Keywords::I8
                    | Keywords::I32
                    | Keywords::F32
                    | Keywords::Char => {
                        let Ok(_) = curr_paramater.set_type(kw.into()) else {
                            return Err(
                                format!(
                                "A paramater type was either assigned after the name or twice on line {line}"
                            ))
                        };
                    }
                    _ => {
                        return Err(format!(
                            "Expected the type of paramter to be valid but got {kw:#?}"
                        ))
                    }
                },
                Tokens::Identifier => {
                    let Ok(_) = curr_paramater.set_name(&tokens.value) else {
                        return Err("tried assigning a name to a variable that already has a name".to_owned());
                    };
                }
                Tokens::Comma => {
                    let is_valid = curr_paramater.is_valid();
                    if is_valid.is_err() {
                        return Err(format!(
                            "Paramater one line {line} isn't a valid paramater because {}",
                            is_valid.err().unwrap()
                        ));
                    }
                    paramaters.push(curr_paramater.clone());
                    curr_paramater.clear()
                }
                Tokens::CloseBrace => {
                    let is_valid = curr_paramater.is_valid();
                    if is_valid.is_ok() {
                        paramaters.push(curr_paramater);
                    }
                    return Ok(paramaters);
                }
                token => return Err(ParseErrors::InvalidToken(line, token).to_string()),
            }
        }

        return Err(format!(
            "Expected a ending close brace for paramaters on line {line}, but didn't get one"
        ));
    }

    fn parse_function_return_type(&mut self, line: usize) -> Result<MarkerTypes, String> {
        let Some(return_type) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string())
        };
        match return_type.token_type {
            Tokens::Kw(keyword) => Ok(MarkerTypes::from(keyword)),
            token => {
                return Err(format!(
                    "Expected a return type on line {line} but got {token:#?}"
                ))
            }
        }
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        let Some(function_name) = self.next() else {
            return Err("Expected the previous token to be some".to_string());
        };
        let function_name = function_name.clone();
        let params = self.parse_params()?;
        let return_type = self.parse_function_return_type(function_name.line)?;
        return Ok(Function {
            name: function_name.value,
            block: self.parse_block()?,
            return_type,
            params,
            line: function_name.line,
        });
    }
}
