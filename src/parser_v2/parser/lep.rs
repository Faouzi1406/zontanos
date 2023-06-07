use crate::parser_v2::ast::{NodeTypes, Value};
use crate::parser_v2::parser::ParseResult;
use crate::parser_v2::parser::Parser;
use crate::zon_parser::lexer::{Operator, Tokens};

pub enum Statements {
    Or,
    And,
    More(Value, Value),
    Less(Value, Value),
    MoreEq(Value, Value),
    LessEq(Value, Value),
    OrOr(Value, Value),
    AndAnd(Value, Value),
    Atomic(Value),
}

pub struct LogicalStatement {
    case: Vec<Statements>,
    if_do: NodeTypes,
    else_do: Option<NodeTypes>,
}

impl Parser {
    fn lep_parse(&mut self) -> ParseResult<LogicalStatement> {
        todo!("Parse logical expressions")
    }

    pub fn lep_parse_statements(&mut self) -> ParseResult<Vec<Statements>> {
        let mut statements = Vec::new();
        while let Some(logical_expr_token) = self.next() {
            self.walk_back(1);
            // We expect the first token to be that of a value
            let value = self.parse_not_know_type_value()?;

            // We expect there to be some operator | token next
            let Some(operator) = self.next() else {
                return Err(self.lep_parse_expected_op());
            };

            // Token is and operator, this could mean there is and '||' | '!='...
            if let Tokens::Op(op) = operator.token_type {
                match op {
                    Operator::More => {
                        let other_value = self.parse_not_know_type_value()?;
                        statements.push(Statements::More(value, other_value));
                    }
                    Operator::Less => {
                        let other_value = self.parse_not_know_type_value()?;
                        statements.push(Statements::Less(value, other_value));
                    }
                    Operator::MoreEq => {
                        let other_value = self.parse_not_know_type_value()?;
                        statements.push(Statements::MoreEq(value, other_value));
                    }
                    Operator::LessEq => {
                        let other_value = self.parse_not_know_type_value()?;
                        statements.push(Statements::MoreEq(value, other_value));
                    }
                    Operator::OrOr => {
                        let other_value = self.parse_not_know_type_value()?;
                        statements.push(Statements::OrOr(value, other_value));
                    }
                    Operator::AndAnd => {
                        let other_value = self.parse_not_know_type_value()?;
                        statements.push(Statements::AndAnd(value, other_value));
                    }
                    _ => return Err(self.lep_unexpected_token()),
                } continue;
            };

            if let Some(token_continue_op) = self.next() {
                if let Tokens::Op(op) = token_continue_op.token_type {
                    match op {
                        Operator::AndAnd => {
                            statements.push(Statements::Atomic(value));
                            statements.push(Statements::Or);
                        }
                        Operator::OrOr => {
                            statements.push(Statements::Atomic(value));
                            statements.push(Statements::Or);
                        }
                        _ => return Err(self.lep_unexpected_token())
                    }
                }
            };
            continue;
        }
        Err(self.lep_expected_lep_or_end())
    } // TODO: Finish this function tommorow, getting kinda late right now.

    fn lep_single_op_tokens(&mut self) -> ParseResult<Statements> {
        match self.next() {
            Some(value) => match value.token_type {
                Tokens::Op(op) => match op {
                    Operator::AndAnd => Ok(Statements::And),
                    Operator::OrOr => Ok(Statements::Or),
                    token => Err(self.lep_unexpected_token()),
                },
                _ => Err("single op token cannot parse other token_types then operators".into()),
            },
            None => Err("single op token expected there to be  a token".to_string()),
        }
    }

    fn lep_parse_expected_op(&mut self) -> String {
        let prev = self.assert_prev_token();
        let msg = format!("expected the next token to be and operator or atleast a close brace, if there is not more cases, found on line {}", prev.line);
        msg
    }

    fn lep_parse_expected_end(&mut self) -> String {
        let prev = self.assert_prev_token();
        let msg = format!(
            "expected a end to the logical expression on line {}",
            prev.line
        );
        msg
    }

    fn lep_expected_lep_or_end(&mut self) -> String {
        let prev = self.assert_prev_token();
        let msg = format!(
            "expected a end to the logical expression ')', or a logical expression. on line {}",
            prev.line
        );
        msg
    }

    fn lep_unexpected_token(&mut self) -> String {
        let token = self.assert_prev_token();
        format!("Got a token: {} that does not belong in the context of a logical expression. on line: {}", token.value, token.line)
    }
}
