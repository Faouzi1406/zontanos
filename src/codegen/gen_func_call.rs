use std::{println, todo, unimplemented};

use super::{std_gen::std_functions::StdFunctions, CodeGen};
use crate::ast::{
    function_call::FunctionCall,
    types::{MarkerTypes, VarTypes},
};
use inkwell::{
    types::{AnyTypeEnum, BasicTypeEnum, PointerType},
    values::{
        AnyValue, AsValueRef, BasicMetadataValueEnum, CallSiteValue, FunctionValue,
        InstructionValue, PointerValue,
    },
    AddressSpace,
};

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn gen_named_function_call(
        &self,
        call: &'ctx FunctionCall,
        scope: FunctionValue<'ctx>,
        name: &str,
        expected_return: &MarkerTypes,
    ) -> Result<(), String> {
        let arguments = &call.args;
        let arguments = self.get_call_args(arguments, scope)?;

        if let Some(func_call) = self.module.get_function(&call.call_to) {
            let params = func_call.get_params();
            let call_return = self.builder.build_call(func_call, &arguments, name);
            if !self.match_call_return_type(expected_return, call_return) {
                return Err(format!(
                    "Found a call to {} from {name} but the expected return type {} wasn't the return type of {}",
                    &call.call_to,
                    expected_return.to_string(),
                    &call.call_to
                ));
            }
            return Ok(());
        };

        let return_value = self.gen_std_func(arguments, scope, Some(name), &call.call_to)?;
        if &return_value != expected_return {
            return Err(format!("Couldn't compile call to {} since the return type of {name} is not the expected return type of {}", name, expected_return.to_string()));
        }

        return Ok(());
    }

    pub(super) fn get_call_args(
        &self,
        arguments: &Vec<VarTypes>,
        scope: FunctionValue<'ctx>,
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
                    let int_value = u8.const_int(*value_u8 as u64, false);
                    value_vec.push(int_value.into());
                }
                VarTypes::I8(value_i8) => {
                    let i8 = self.context.i8_type();
                    let int_value = i8.const_int(*value_i8 as u64, false);
                    value_vec.push(int_value.into());
                }
                VarTypes::I32(value_i32) => {
                    let i32 = self.context.i32_type();
                    let int_value = i32.const_int(*value_i32 as u64, false);
                    value_vec.push(int_value.into());
                }
                VarTypes::F32(value_f32) => {
                    let f32 = self.context.f32_type();
                    let int_value = f32.const_float(*value_f32 as f64);
                    value_vec.push(int_value.into());
                }
                VarTypes::Char(char) => {
                    let i8 = self.context.i8_type();
                    let int_value = i8.const_int(*char as u64, false);
                    value_vec.push(int_value.into());
                }
                VarTypes::Identifier(id, expected_type) => {
                    let Some(block) = scope.get_first_basic_block() else {
                        return Err(format!("Found a identifier argument {id} but couldn't access any scope therefore {id} can't exist within the current scope."));
                    };
                    let Some(variable) = block.get_instruction_with_name(&id) else {
                        return Err(format!("Found a identifier argument {id} but couldn't find it, perhaps you made a naming mistake."));
                    };
                    let value = self.value_from_instruction(variable);
                    value_vec.push(value);
                }
                VarTypes::Array { array, array_type } => {}
                VarTypes::FunctionCall(call, expected_type) => {}
                _ => unimplemented!("boolean values"),
            }
        }

        Ok(value_vec)
    }

    /// returns false if the expected return type was not the same as the function call return type
    pub(super) fn match_call_return_type(
        &self,
        with: &MarkerTypes,
        against: CallSiteValue,
    ) -> bool {
        if let Some(call_return_type) = against.get_called_fn_value().get_type().get_return_type() {
            match call_return_type {
                BasicTypeEnum::IntType(_) => *with == MarkerTypes::I32,
                BasicTypeEnum::FloatType(_) => *with == MarkerTypes::F32,
                BasicTypeEnum::ArrayType(..) => todo!("Arrays as return types are not yet supported in the language"),
                _ => unimplemented!("this type is not supported in the langaue and therefore shouldn't be able to be a return value"),
            }
        } else {
            false
        }
    }

    pub(super) fn value_from_instruction(
        &self,
        instruction_value: InstructionValue<'ctx>,
    ) -> BasicMetadataValueEnum {
        let value = instruction_value.as_any_value_enum();
        match instruction_value.get_type() {
            AnyTypeEnum::IntType(int) => value.into_int_value().into(),
            AnyTypeEnum::FloatType(float) => value.into_float_value().into(),
            AnyTypeEnum::PointerType(ptr) => self.load_pointer(value.into_pointer_value(), ptr),
            AnyTypeEnum::ArrayType(arr) => value.into_array_value().into(),
            type_passed => unimplemented!(
                "passing type of {type_passed:#?} as a paramater is not yet supported"
            ),
        }
    }

    pub(super) fn load_pointer(
        &self,
        ptr_value: PointerValue<'ctx>,
        ptr_type: PointerType,
    ) -> BasicMetadataValueEnum {
        match ptr_type.get_element_type() {
            AnyTypeEnum::ArrayType(_) => ptr_value.into(),
            _ => self.builder.build_load(ptr_value, "load").into(),
        }
    }
}
