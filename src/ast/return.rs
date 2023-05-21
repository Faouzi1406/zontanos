use super::types::VarTypes;

/// The return structs, marks a return value
#[derive(Debug, PartialEq)]
pub struct Return(pub VarTypes);

impl Return {
    pub fn new(var_type: VarTypes) -> Self {
        Self(var_type)
    }
}
