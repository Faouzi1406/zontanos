#![allow(unused)]
use std::{io::ErrorKind, todo};

use crate::{
    ast::{
        logic::{Case, LogicalStatements, Statement},
        types::VarTypes,
        Expr,
    },
    zon_parser::{
        lexer::{Keywords, Tokens},
        parser::{parse_errors::ParseErrors, parser::ParseTokens},
    },
};

use super::{
    lexer::{Operator, Token},
    parser::parser::Parser,
};

pub trait IfElseParser {
    fn parse_if(&mut self, line: usize) -> Result<Expr, String>;
    fn parse_else(&mut self) -> Expr;
    fn parse_statements(&mut self, line: usize) -> Result<Vec<LogicalStatements>, String>;
    fn parse_case(&mut self, line: usize) -> Result<Case, String>;
}

trait Helpers {
    fn parse_value(token: Token) -> Result<VarTypes, String>;
    fn parse_operator(token: Token) -> Result<Operator, String>;
}

impl IfElseParser for Parser {
    fn parse_if(&mut self, line: usize) -> Result<Expr, String> {
        let Some(if_statement) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };

        if if_statement.token_type != Tokens::Kw(Keywords::If) {
            return Err(
                ParseErrors::WrongToken(Tokens::Kw(Keywords::If), if_statement.token_type)
                    .to_string(),
            );
        }
        let statements = self.parse_statements(line)?;
        let parse_block = self.parse_block()?;
        let if_statement = Statement {
            if_true: parse_block,
            if_false: None,
            statements,
        };
        Ok(Expr::Logic(if_statement))
    }
    fn parse_else(&mut self) -> Expr {
        todo!()
    }
    fn parse_statements(&mut self, line: usize) -> Result<Vec<LogicalStatements>, String> {
        let Some(open_brace) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };
        if open_brace.token_type != Tokens::OpenBrace {
            return Err(
                ParseErrors::WrongToken(Tokens::OpenBrace, open_brace.token_type).to_string(),
            );
        }

        let mut all_cases = Vec::new();
        let mut parse_case = Some(self.parse_case(line)?);

        while let Some(next_statement) = self.next() {
            match next_statement.token_type {
                Tokens::Identifier | Tokens::String | Tokens::Number | Tokens::FloatNumber => {
                    self.advance_back(1);
                    parse_case = Some(self.parse_case(next_statement.line)?);
                }
                Tokens::Op(Operator::OrOr) => {
                    let Some(case)  = parse_case.clone()  else {
                        all_cases.push(LogicalStatements::OrNext);
                        continue;
                    };
                    let or_case = self.parse_case(line)?;
                    all_cases.push(LogicalStatements::Or(case, or_case));
                    parse_case = None;
                }
                Tokens::Op(Operator::AndAnd) => {
                    let Some(case)  = parse_case.clone() else {
                        all_cases.push(LogicalStatements::AndNext);
                        continue;
                    };
                    let and_case = self.parse_case(line)?;
                    all_cases.push(LogicalStatements::And(case, and_case));
                    parse_case = None;
                }
                Tokens::CloseBrace => {
                    if let Some(parse_case) = parse_case {
                        all_cases.push(LogicalStatements::Atomic(parse_case));
                    }
                    return Ok(all_cases);
                }
                invalid_token => {
                    return Err(ParseErrors::InvalidToken(line, invalid_token).to_string())
                }
            }
        }
        Ok(all_cases)
    }
    fn parse_case(&mut self, line: usize) -> Result<Case, String> {
        let mut case = Case::default();
        let Some(type_value) = self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };
        let first_val = Parser::parse_value(type_value)?;
        case.value_1 = first_val;

        let Some(operator) =  self.next() else {
            return Err(ParseErrors::ExpectedNext(line).to_string());
        };
        let Ok(parse_op) = Parser::parse_operator(operator) else {
            self.advance_back(1);
            return Ok(case);
        };

        match parse_op {
            Operator::EqEq
            | Operator::MoreEq
            | Operator::LessEq
            | Operator::Less
            | Operator::More => {
                let Some(second_value)  = self.next() else {
                    return Err(ParseErrors::ExpectedNext(line).to_string());
                };
                let second_value = Parser::parse_value(second_value)?;
                case.value_2 = Some(second_value);
                case.operator = Some(parse_op);
                return Ok(case);
            }
            invalid_token => {
                return Err(ParseErrors::InvalidToken(line, Tokens::Op(invalid_token)).to_string())
            }
        }
    }
}

impl Helpers for Parser {
    fn parse_value(token: Token) -> Result<VarTypes, String> {
        match token.token_type {
            Tokens::String
            | Tokens::Number
            | Tokens::FloatNumber
            | Tokens::Char
            | Tokens::Identifier => {
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
    fn parse_operator(token: Token) -> Result<Operator, String> {
        match token.token_type {
            Tokens::Op(op) => Ok(op),
            invalid_token => Err(ParseErrors::InvalidToken(token.line, invalid_token).to_string()),
        }
    }
}
