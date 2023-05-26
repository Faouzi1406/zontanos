use std::{error::Error, println};

use inkwell::{
    types::BasicType,
    values::{ArrayValue, BasicMetadataValueEnum, BasicValue, FunctionValue, IntValue},
};

use super::CodeGen;
use crate::ast::{
    function::{Function, Paramater},
    types::{MarkerTypes, VarTypes},
    variable::{VarData, Variable},
};

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn gen_scoped_var(
        &self,
        variable: &'ctx Variable,
        scope: FunctionValue<'ctx>,
        func: Option<&'ctx Function>,
    ) -> Result<(), String> {
        let Some(variable_name) = variable.get_name() else {
            return Err(Self::variable_no_name(variable.var_line));
        };

        match &variable.var_type {
            VarTypes::None => Err(Self::variable_has_no_type(variable_name, variable.var_line)),
            VarTypes::Array { array, array_type } => self.gen_array(variable),
            VarTypes::U8(u8) => {
                let i8 = self.context.i8_type();
                let int_value = i8.const_int(*u8 as u64, false);
                self.create_variable(i8, variable_name.to_string(), int_value);
                Ok(())
            }
            VarTypes::I8(i8_value) => {
                let i8 = self.context.i8_type();
                let int_value = i8.const_int(*i8_value as u64, false);
                self.create_variable(i8, variable_name.to_string(), int_value);
                Ok(())
            }
            VarTypes::I32(i32_value) => {
                let i32 = self.context.i32_type();
                let int_value = i32.const_int(*i32_value as u64, false);
                self.create_variable(i32, variable_name.to_string(), int_value);
                Ok(())
            }
            VarTypes::F32(f32_value) => {
                let f32 = self.context.f32_type();
                let int_value = f32.const_float(*f32_value as f64);
                self.create_variable(f32, variable_name.to_string(), int_value);
                Ok(())
            }
            VarTypes::String(str) => {
                let string = self.context.const_string(str.as_bytes(), false);
                let string_type = string.get_type();
                self.create_variable(string_type, variable_name.to_string(), string);
                Ok(())
            }
            VarTypes::FunctionCall(call, expected_type) => {
                let gen_func_call =
                    self.gen_named_function_call(call, scope, variable_name, &expected_type, func)?;
                Ok(())
            }
            _ => unimplemented!(),
        }
    }

    pub(super) fn gen_const_string_variable(&'ctx self, str: String) -> Result<ArrayValue, String> {
        let string = self.context.const_string(str.as_bytes(), false);
        return Ok(string);
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

    pub(super) fn get_value_from_id(
        &self,
        scope: FunctionValue<'ctx>,
        id: &str,
        func_params: Option<&'ctx Vec<Paramater>>,
    ) -> Option<BasicMetadataValueEnum<'ctx>> {
        let Some(block) = scope.get_first_basic_block() else {
            return None;
        };

        if let Some(variable) = block.get_instruction_with_name(&id) {
            let value = self.value_from_instruction(variable);
            return Some(value);
        };

        if let Some(param_index) = func_params
            .unwrap()
            .iter()
            .enumerate()
            .find(|x| x.1.name == *id)
        {
            let Some(get_param) = scope.get_nth_param(param_index.0 as u32) else {
                return None;
            };
            return Some(get_param.into());
        }
        None
    }

    /// Takes in a int BasicMetadataValueEnum; you have to check if it's and int value before you
    /// use this.
    pub(super) fn gen_basic_value_from_meta_value_int(
        &self,
        value: BasicMetadataValueEnum<'ctx>,
    ) -> Option<impl BasicValue> {
        assert!(value.is_int_value());
        match value {
            BasicMetadataValueEnum::IntValue(_) => Some(value.into_int_value()),
            _ => None,
        }
    }

    /// Takes in a float BasicMetadataValueEnum; you have to check if it's and float value before you
    /// use this.
    pub(super) fn gen_basic_value_from_meta_value_float(
        &self,
        value: BasicMetadataValueEnum<'ctx>,
    ) -> Option<impl BasicValue> {
        assert!(value.is_float_value());
        match value {
            BasicMetadataValueEnum::FloatValue(_) => Some(value.into_float_value()),
            _ => None,
        }
    }

    /// Takes in a array BasicMetadataValueEnum; you have to check if it's and array value before you
    /// use this.
    pub(super) fn gen_basic_value_from_meta_value_array(
        &self,
        value: BasicMetadataValueEnum<'ctx>,
    ) -> Option<impl BasicValue> {
        assert!(value.is_array_value());
        match value {
            BasicMetadataValueEnum::ArrayValue(_) => Some(value.into_array_value()),
            _ => None,
        }
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

    pub(super) fn expected_string_value(line: usize) -> String {
        let msg = format!(
            "Tried to create a string but the variable type was not of string,  line: {line}"
        );
        msg
    }
}
