use crate::ast::{function_call::FunctionCall, types::VarTypes};
use inkwell::values::{
    AnyValue, AsValueRef, BasicMetadataValueEnum, FunctionValue, InstructionValue,
};

use super::{std_gen::std_functions::StdFunctions, CodeGen};

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn gen_named_function_call(
        &self,
        call: &'ctx FunctionCall,
        scope: FunctionValue,
        name: &str,
    ) -> Result<(), String> {
        let arguments = &call.args;
        let arguments = self.get_call_args(arguments.to_vec(), scope)?;

        if let Some(func_call) = self.module.get_function(&call.call_to) {
            let params = func_call.get_params();
            self.builder.build_call(func_call, &arguments, name);
        };

        self.gen_std_func(arguments, scope, Some(name), &call.call_to)?;

        return Err(format!(
            "There is no function with the name: {} on line: {}",
            &call.call_to, call.line
        ));
    }

    pub(super) fn get_call_args(
        &self,
        arguments: Vec<VarTypes>,
        scope: FunctionValue,
    ) -> Result<Vec<BasicMetadataValueEnum>, String> {
        let mut value_vec = Vec::new();

        for value in arguments {
            match value {
                VarTypes::String(value_str) => {
                    let string_value = self.builder.build_global_string_ptr(&value_str, "call_str");
                    value_vec.push(string_value.as_pointer_value().into());
                }
                VarTypes::U8(value_u8) => {
                    let u8 = self.context.i8_type();
                    let int_value = u8.const_int(value_u8 as u64, false);
                    value_vec.push(int_value.into());
                }
                VarTypes::I8(value_i8) => {
                    let i8 = self.context.i8_type();
                    let int_value = i8.const_int(value_i8 as u64, false);
                    value_vec.push(int_value.into());
                }
                VarTypes::I32(value_i32) => {
                    let i32 = self.context.i32_type();
                    let int_value = i32.const_int(value_i32 as u64, false);
                    value_vec.push(int_value.into());
                }
                VarTypes::F32(value_f32) => {
                    let f32 = self.context.f32_type();
                    let int_value = f32.const_float(value_f32 as f64);
                    value_vec.push(int_value.into());
                }
                VarTypes::Char(char) => {
                    let i8 = self.context.i8_type();
                    let int_value = i8.const_int(char as u64, false);
                    value_vec.push(int_value.into());
                }
                VarTypes::Identifier(id, expected_type) => {
                    todo!("Identifiers")
                }
                VarTypes::Array { array, array_type } => {}
                VarTypes::FunctionCall(call, expected_type) => {}
                _ => unimplemented!("boolean values"),
            }
        }

        Ok(value_vec)
    }

    pub(super) fn value_from_instruction(instruction_value: InstructionValue) {}
}
