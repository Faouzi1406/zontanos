use std::error::Error;

use inkwell::{types::BasicType, values::BasicValue};

use super::CodeGen;
use crate::ast::{
    types::{MarkerTypes, VarTypes},
    variable::{VarData, Variable},
};

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn gen_var(&'ctx self, variable: &'ctx Variable) -> Result<(), String> {
        let Some(variable_name) = variable.get_name() else {
            return Err(Self::variable_no_name(variable.var_line));
        };

        match &variable.var_type {
            VarTypes::None => Err(Self::variable_has_no_type(variable_name, variable.var_line)),
            VarTypes::Array { array, array_type } => self.gen_array(&variable),
            _ => unimplemented!(),
        }
    }

    pub(super) fn create_variable<T: BasicType<'ctx>, V: BasicValue<'ctx>>(
        &self,
        type_value: T,
        var_name: String,
        value: V,
    ) {
        let ptr = self.builder.build_alloca(type_value, &var_name);
        self.builder.build_store(ptr, value);
    }

    /// Error message if a variable with no name is found during parsing
    pub(super) fn variable_no_name(line: usize) -> String {
        let msg = format!("Tried to generate a variable without a name; line: {line}");
        msg
    }

    pub(super) fn variable_has_no_type(name: &str, line: usize) -> String {
        let msg = format!("Found a variable without a type let {name}:T = T; line: {line}");
        msg
    }
}
