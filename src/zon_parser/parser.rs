//! zon_parser/parser.rs
//!
//! This is the main parser of the zontanos;
//! It will be responsible for turning the tokens into the Ast.
//! It should detect whenever there is a invalid set of tokens.

use crate::ast::Ast;

use super::lexer::{Keywords, Token, Tokens};

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

pub trait Parse {
    fn new(tokens: Vec<Token>) -> Self;
    fn parse(&mut self) -> Result<Ast, Vec<String>>;
    fn tokens_until_mut(&mut self, tokens: Tokens) -> Option<Vec<Token>>;
    fn tokens_until(&self, until_token: Tokens) -> Option<Vec<Token>>;
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
