use crate::parser_v2::parser::ParseResult;
use crate::parser_v2::ast::{NodeTypes, Value};
use crate::parser_v2::parser::Parser;

pub enum Statements {
    Or,
    And,
    OrCase(Value, Value),
    AndCase(Value, Value),
    Atomic(Value),
}

pub struct LogicalStatement {
    case: Vec<Statements>,
    if_do: NodeTypes,
    else_do: Option<NodeTypes>,
}

impl Parser {
    fn lep_parse(&mut self) -> ParseResult<Vec<LogicalStatement>> {
        let statements = Vec::new();
        while let Some(logical_expr_token) = self.next() {
            let value = self.parse_not_know_type_value();
            
        }
        Ok(statements)
    }
}
