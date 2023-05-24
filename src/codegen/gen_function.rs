use std::error::Error;
use std::{unimplemented, unreachable};

use inkwell::basic_block::BasicBlock;
use inkwell::types::{BasicMetadataTypeEnum, FunctionType};
use inkwell::values::FunctionValue;

use crate::ast::function::Function;
use crate::ast::r#return::Return;
use crate::ast::types::MarkerTypes;

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn gen_function(&self, function: Function) -> Result<(FunctionValue,BasicBlock), String> {
        let return_value = self.fn_return_value(&function);
        let function = self.module.add_function(&function.name, return_value, None);
        let block = self.context.append_basic_block(function, "entry");
        return Ok((function, block));
    }

    fn fn_return_value(&self, function: &Function) -> FunctionType<'ctx> {
        let params = self.get_params_function(function);
        match &function.return_type {
            MarkerTypes::Array(typeof_array) => {
                unimplemented!("support for arrays as return values is not yet supported")
            }
            MarkerTypes::U8 => self.context.i8_type().fn_type(&params, false),
            MarkerTypes::I8 => self.context.i8_type().fn_type(&params, false),
            MarkerTypes::I32 => self.context.i32_type().fn_type(&params, false),
            MarkerTypes::F32 => self.context.f32_type().fn_type(&params, false),
            MarkerTypes::Char => self.context.i8_type().fn_type(&params, false),
            MarkerTypes::String => self.context.i8_type().fn_type(&params, false),
            MarkerTypes::Identifier => self.context.void_type().fn_type(&params, false),
            MarkerTypes::Void => self.context.void_type().fn_type(&params, false),
            MarkerTypes::None => self.context.void_type().fn_type(&params, false),
        }
    }

    fn get_params_function(&self, function: &Function) -> Vec<BasicMetadataTypeEnum<'ctx>> {
        let mut vec = Vec::new();
        for param in &function.params {
            match &param.paramater_type {
                MarkerTypes::Array(_) => todo!("Add arrays for params"),
                MarkerTypes::String => vec.push(self.context.i8_type().into()),
                // This should be fine since the character is static thus conversion shouldn't
                // cause any problems since those would be catched by the parser.
                MarkerTypes::Char => vec.push(self.context.i8_type().into()),
                MarkerTypes::I32 => vec.push(self.context.i32_type().into()),
                MarkerTypes::F32 => vec.push(self.context.f32_type().into()),
                // Yeah have to think about how to fix this?
                MarkerTypes::U8 => vec.push(self.context.i8_type().into()),
                MarkerTypes::I8 => vec.push(self.context.i8_type().into()),
                non_type => {
                    unreachable!("Found a {non_type:#?} type as paramter...")
                }
            }
        }
        vec
    }

    pub(super) fn gen_function_return(&self, ret: &Return) {
        if ret.is_int_return() {
            if let Some(int_return) = self.get_int_return_value(ret) {
                self.builder.build_return(Some(&int_return));
            };
        }
        if ret.is_float_return() {
            if let Some(int_return) = self.gen_float_return_type(ret) {
                self.builder.build_return(Some(&int_return));
            };
        }
        if ret.is_array_return() {
            if let Some(int_return) = self.gen_arr_return_type(ret) {
                self.builder.build_return(Some(&int_return));
            };
        }
    }
}
