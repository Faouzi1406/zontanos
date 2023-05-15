use super::{function::Function, types::VarTypes};

pub struct FunctionCall {
    call_to: Function,
    args: Vec<VarTypes>,
    line: u32,
}
