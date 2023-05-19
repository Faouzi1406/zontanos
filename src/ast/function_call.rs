use super::types::VarTypes;

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub call_to: String,
    pub args: Vec<VarTypes>,
    pub line: usize,
}
