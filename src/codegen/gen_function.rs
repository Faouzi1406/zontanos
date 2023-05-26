use std::error::Error;
use std::{unimplemented, unreachable};

use inkwell::basic_block::BasicBlock;
use inkwell::types::{BasicMetadataTypeEnum, FunctionType};
use inkwell::values::{AnyValue, FunctionValue, AnyValueEnum};
use inkwell::AddressSpace;

use crate::ast::function::Function;
use crate::ast::function_call::FunctionCall;
use crate::ast::r#return::{self, Return};
use crate::ast::types::{MarkerTypes, VarTypes};

use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn gen_function(
        &self,
        function: &Function,
    ) -> Result<(FunctionValue<'ctx>, BasicBlock<'ctx>), String> {
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
            MarkerTypes::PointerI8 => self
                .context
                .i8_type()
                .ptr_type(Default::default())
                .fn_type(&params, false),
            MarkerTypes::PointerU8 => self
                .context
                .i8_type()
                .ptr_type(Default::default())
                .fn_type(&params, false),
            MarkerTypes::PointerChar => self
                .context
                .i8_type()
                .ptr_type(Default::default())
                .fn_type(&params, false),
            MarkerTypes::PointerString => self
                .context
                .i8_type()
                .ptr_type(Default::default())
                .fn_type(&params, false),
            MarkerTypes::PointerI32 => self
                .context
                .i32_type()
                .ptr_type(Default::default())
                .fn_type(&params, false),
            MarkerTypes::Void => self.context.void_type().fn_type(&params, false),
            MarkerTypes::None => self.context.void_type().fn_type(&params, false),
            value => todo!("{:#?}", value),
        }
    }

    fn get_params_function(&self, function: &Function) -> Vec<BasicMetadataTypeEnum<'ctx>> {
        let mut vec = Vec::new();
        for param in &function.params {
            match &param.paramater_type {
                MarkerTypes::Array(_) => todo!("Add arrays for params"),
                MarkerTypes::String => vec.push(
                    self.context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .into(),
                ),
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

    pub(super) fn gen_function_return(
        &self,
        ret: &'ctx Return,
        scope: FunctionValue<'ctx>,
        func: &'ctx Function,
    ) -> Result<(), String> {
        if let VarTypes::FunctionCall(call, _) = &ret.0 {
            self.gen_return_from_call(call, scope, func)?;
            return Ok(());
        }

        if let VarTypes::Identifier(id, type_id) = &ret.0 {
            if let Some(value) = self.get_value_from_id(scope, id, Some(&func.params)) {
                if value.is_int_value() {
                    self.builder.build_return(Some(&value.into_int_value()));
                }
                if value.is_array_value() {
                    self.builder.build_return(Some(&value.into_array_value()));
                }
                if value.is_float_value() {
                    self.builder.build_return(Some(&value.into_float_value()));
                }
                if value.is_pointer_value() {
                    let ptr = value.into_pointer_value();
                    let value = self.load_pointer(ptr, ptr.get_type());

                    if value.is_pointer_value() {
                        self.builder.build_return(Some(&value.into_pointer_value()));
                    }
                    if value.is_int_value() {
                        self.builder.build_return(Some(&value.into_int_value()));
                    }
                    if value.is_array_value() {
                        self.builder.build_return(Some(&value.into_array_value()));
                    }
                }
                return Ok(());
            };
            return Err(format!(
                "No identifier with the name {id} found in function {}",
                func.name
            ));
        }

        if ret.is_int_return() {
            if let Some(int_return) = self.get_int_return_value(ret) {
                self.builder.build_return(Some(&int_return));
                return Ok(());
            };
        }
        if ret.is_float_return() {
            if let Some(int_return) = self.gen_float_return_value(ret) {
                self.builder.build_return(Some(&int_return));
                return Ok(());
            };
        }
        if ret.is_array_return() {
            if let Some(int_return) = self.gen_arr_return_value(ret) {
                self.builder.build_return(Some(&int_return));
                return Ok(());
            };
        }

        Err("function return type was not of any know return type".to_string())
    }

    pub(super) fn gen_return_from_call(
        &self,
        call: &'ctx FunctionCall,
        scope: FunctionValue<'ctx>,
        func: &'ctx Function,
    ) -> Result<(), String> {
        let return_value =
            self.gen_named_function_call(call, scope, "return", &func.return_type, Some(func))?;
        let value = match return_value {
            AnyValueEnum::IntValue(value) => self.builder.build_return(Some(&value)),
            AnyValueEnum::FloatValue(value) => self.builder.build_return(Some(&value)),
            AnyValueEnum::PointerValue(value) => self.builder.build_return(Some(&value)),
            value => todo!("Support the call return type of {value}")
        };
        
        return Ok(());
    }
}
