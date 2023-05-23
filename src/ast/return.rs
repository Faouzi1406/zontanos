use super::types::VarTypes;

/// The return structs, marks a return value
#[derive(Debug, PartialEq, Clone)]
pub struct Return(pub VarTypes);

impl Return {
    pub fn new(var_type: VarTypes) -> Self {
        Self(var_type)
    }
}

impl Return {
    pub fn is_int_return(&self) -> bool {
        match &self.0 {
            VarTypes::U8(_) => true,
            VarTypes::I8(_) => true,
            VarTypes::I32(_) => true,
            _ => false,
        }
    }

    pub fn is_float_return(&self) -> bool {
        match &self.0 {
            VarTypes::F32(_) => true,
            _ => false,
        }
    }

    pub fn is_array_return(&self) -> bool {
        match &self.0 {
            VarTypes::String(_) => true,
            VarTypes::Array { .. } => true,
            _ => false,
        }
    }
}
