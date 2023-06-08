use crate::parser_v2::ast::{NodeTypes, Type, Value};
use crate::parser_v2::parser::ParseResult;
use crate::parser_v2::parser::Parser;
use crate::zon_parser::lexer::{Keywords, Operator, Tokens};

#[derive(Debug, PartialEq)]
pub enum Statements {
    Or,
    And,
    EqEq(Value, Value),
    More(Value, Value),
    Less(Value, Value),
    MoreEq(Value, Value),
    LessEq(Value, Value),
    OrOr(Value, Value),
    AndAnd(Value, Value),
    Atomic(Value),
}

#[derive(Debug)]
pub struct LogicalStatement {
    pub case: Vec<Statements>,
    pub if_do: NodeTypes,
    pub else_do: Option<NodeTypes>,
}

impl LogicalStatement {
    fn new(case: Vec<Statements>, if_do: NodeTypes, else_do: Option<NodeTypes>) -> Self {
        Self {
            case,
            if_do,
            else_do,
        }
    }
}

impl Parser {
    pub fn lep_parse(&mut self, type_expected: &Type) -> ParseResult<LogicalStatement> {
        let statements = self.lep_parse_statements()?;
        let if_block = self.parse_block_expr(type_expected)?;
        let else_block = if self.consume_if_next(Tokens::Kw(Keywords::Else)) {
            Some(NodeTypes::Block(self.parse_block_expr(type_expected)?.0))
        } else {
            None
        };
        Ok(LogicalStatement::new(
            statements,
            NodeTypes::Block(if_block.0),
            else_block,
        ))
    }

    pub fn lep_parse_statements(&mut self) -> ParseResult<Vec<Statements>> {
        let mut statements = Vec::new();
        while let Some(_) = self.next() {
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
                    Operator::EqEq => {
                        let other_value = self.parse_not_know_type_value()?;
                        statements.push(Statements::EqEq(value, other_value));
                    }
                    _ => return Err(self.lep_unexpected_token()),
                }
            } else {
                statements.push(Statements::Atomic(value));
                self.walk_back(1);
            };

            if let Some(token_continue_op) = self.next() {
                if token_continue_op.token_type == Tokens::OpenCurlyBracket {
                    self.walk_back(1);
                    return Ok(statements);
                }
                if let Tokens::Op(op) = token_continue_op.token_type {
                    match op {
                        Operator::AndAnd => {
                            statements.push(Statements::And);
                        }
                        Operator::OrOr => {
                            statements.push(Statements::Or);
                        }
                        _ => return Err(self.lep_unexpected_token()),
                    }
                } 
                continue;
            };
            return Err(self.lep_expected_lep_or_end());
        }
        Err(self.lep_expected_lep_or_end())
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
            "expected a end to the logical expression or a logical expression. on line {}",
            prev.line
        );
        msg
    }

    fn lep_unexpected_token(&mut self) -> String {
        let token = self.assert_prev_token();
        format!("Got a token: {} that does not belong in the context of a logical expression. on line: {}", token.value, token.line)
    }
}
