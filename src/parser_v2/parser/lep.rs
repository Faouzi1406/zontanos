use crate::parser_v2::ast::{NodeTypes, Value};
use crate::parser_v2::parser::Parser;

pub enum Statements {
    OrCase(Value, Value),
    AndCase(Value, Value),
    Or,
    And,
    Atomic(Value),
}

pub struct LogicalStatement {
    case: Vec<Statements>,
    if_do: NodeTypes,
    else_do: Option<NodeTypes>,
}

impl Parser {
    fn lep_parse(&mut self) {
        while let Some(logical_expr_token) = self.next() {
            match logical_expr_token.token_type {
                _ => return,
            }
        }
    }
}
