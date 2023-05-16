//! zon_parser/parser.rs
//!
//! This is the main parser of the zontanos;
//! It will be responsible for turning the tokens into the Ast.
//! It should detect whenever there is a invalid set of tokens.

use crate::{
    ast::{
        variable::{VarData, Variable},
        Ast,
    },
    zon_parser::{lexer::Operator, parser::parse_errors::ParseErrors},
};

use crate::zon_parser::lexer::{Keywords, Token, Tokens};

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
    fn parse_var_assignment(&mut self) -> Result<(), String>;
}

impl ParseTokens for Parser {
    fn parse_var_assignment(&mut self) -> Result<(), String> {
        let Some(prev_token) = self.get_prev_token() else {
            return Err(ParseErrors::NoPrevToken.to_string());
        };
        let prev_token = prev_token.clone();
        if prev_token.token_type != Tokens::Kw(Keywords::Let) {
            return Err(ParseErrors::WrongToken(Tokens::Kw(Keywords::Let), prev_token.token_type).to_string());
        }
        
        let mut variable = Variable::default();
        let Some(var_name) = self.next() else {
            return Err(ParseErrors::ExpectedNext(prev_token.line).to_string());
        };
        if var_name.token_type != Tokens::Identifier {
            return Err(ParseErrors::WrongToken(Tokens::Identifier, var_name.token_type).to_string());
        }

        let _ = variable.set_name(var_name.value, Some(var_name.line));

        return Ok(());
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
        let ast = Ast::new();
        for token in self.by_ref() {
            match token.token_type {
                // We ignore comments
                Tokens::Comment => continue,
                Tokens::Kw(Keywords::Let) => {}
                //Tokens:: Parse tokens
                _ => todo!(),
            }
        }
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
