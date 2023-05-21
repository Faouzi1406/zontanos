use crate::zon_parser::lexer::Operator;

use super::{block::Block, types::VarTypes};

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub statements: Vec<LogicalStatements>,
    pub if_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Case {
    pub value_1: VarTypes,
    pub operator: Option<Operator>,
    pub value_2: Option<VarTypes>,
}

#[derive(Debug, PartialEq)]
pub enum LogicalStatements {
    And(Case, Case),
    AndNext,
    Or(Case, Case),
    OrNext,
    /// Atomics are cases in which one truth isn't joined by others, therefore it has no logical
    /// connectives;
    ///
    /// # Example
    ///
    /// while(true) {
    ///     print(false)
    /// }
    Atomic(Case),
}

impl Default for Case {
    fn default() -> Self {
        Self {
            value_1: VarTypes::None,
            operator: None,
            value_2: None,
        }
    }
}
