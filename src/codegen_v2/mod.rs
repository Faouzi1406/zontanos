#![allow(dead_code)]

use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{ArrayType, BasicMetadataTypeEnum, BasicType};
use inkwell::values::{IntValue, PointerValue};
use inkwell::AddressSpace;
use inkwell::{builder::Builder, values::FunctionValue};
use std::error::Error;

use crate::parser_v2::ast::{
    Ast, Function, Node, NodeTypes, Paramater, Type, TypeValues, Types, Value, Variable, FunctionCall,
};

pub struct CodeGen<'ctx> {
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub context: &'ctx Context,
}

type CompileResult<T> = Result<T, Box<dyn Error>>;

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_ast(&self, ast: &'ctx Ast) -> CompileResult<()> {
        for node in &ast.body {
            match &node.node_type {
                NodeTypes::Function(func) => {
                    let function = self.gen_func(func)?;
                    self.gen_block(function, &func.body)?;
                }
                _ => todo!("compile this node"),
            }
        }
        Ok(())
    }

    fn gen_func(&self, func: &'ctx Function) -> CompileResult<FunctionValue<'ctx>> {
        if func.returns.is_array {
            let return_type = self.gen_type_array(&func.returns)?;
            let params = self.gen_params(&func.paramaters)?;
            let return_type = return_type.fn_type(&params, false);
            Ok(self
                .module
                .add_function(&func.ident.name, return_type, None))
        } else {
            let return_type = self.gen_type(&func.returns)?;
            let params = self.gen_params(&func.paramaters)?;
            let return_type = return_type.fn_type(&params, false);
            Ok(self
                .module
                .add_function(&func.ident.name, return_type, None))
        }
    }

    fn gen_block(
        &self,
        func: FunctionValue<'ctx>,
        nodes: &'ctx Vec<Node>,
    ) -> CompileResult<BasicBlock<'ctx>> {
        let block = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(block);
        for node in nodes {
            match &node.node_type {
                NodeTypes::Variable(var) => {
                    let NodeTypes::Value(value) = &node.right.as_ref().unwrap().node_type else { panic!("expected the right node of a variable to be value") };
                    self.gen_alloca_store(var, value)?;
                }
                NodeTypes::Return(returns) => {
                    let _ = self.gen_return(returns);
                }
                node_type => unimplemented!("Support for {node_type:#?} in blocks is not yet implemented. found this value on line: {}", node.line)
            }
        }
        Ok(block)
    }

    fn gen_return(&self, value: &Value) {
        match &value.value {
            TypeValues::I8(num) => {
                let i8_type = self.context.i8_type();
                let i8_value = i8_type.const_int(*num as u64, false);
                self.builder.build_return(Some(&i8_value));
            }
            TypeValues::U8(num) => {
                let i8_type = self.context.i8_type();
                let i8_value = i8_type.const_int(*num as u64, false);
                self.builder.build_return(Some(&i8_value));
            }
            TypeValues::I32(num) => {
                let i32_type = self.context.i32_type();
                let i32_value = i32_type.const_int(*num as u64, false);
                self.builder.build_return(Some(&i32_value));
            }
            TypeValues::String(str) => {
                let str_array = self.context.i8_type();
                let i8_str = str_array.const_array(&self.str_into_array(&str));
                self.builder.build_return(Some(&i8_str));
                
            }
            typeofval => unimplemented!("typeof {typeofval:#?} is not supported as of right now"),
        };
    }

    fn gen_alloca_store(&self, variable: &'ctx Variable, value: &Value) -> CompileResult<()> {
        if variable.var_type.is_array {
            let arr_type = self.gen_type_array(&variable.var_type)?;
            let alloc = self.builder.build_alloca(arr_type, &variable.ident.name);
            self.gen_store(value, alloc);
        } else {
            let var_type = self.gen_type(&variable.var_type)?;
            let alloc = self.builder.build_alloca(var_type, &variable.ident.name);
            self.gen_store(value, alloc);
        }
        Ok(())
    }

    fn str_into_array(&self, str: &str) -> Vec<IntValue<'ctx>> {
        let iter = str
            .chars()
            .into_iter()
            .map(|value| {
                let i8_type = self.context.i8_type();
                let value = i8_type.const_int(value as u64, false);
                value
            })
            .collect();
        iter
    }

    fn gen_store(&self, value: &Value, alloc_ptr: PointerValue) {
        match &value.value {
            TypeValues::I8(num) => {
                let i8_type = self.context.i8_type();
                let i8_value = i8_type.const_int(*num as u64, false);
                self.builder.build_store(alloc_ptr, i8_value);
            }
            TypeValues::U8(num) => {
                let i8_type = self.context.i8_type();
                let i8_value = i8_type.const_int(*num as u64, false);
                self.builder.build_store(alloc_ptr, i8_value);
            }
            TypeValues::I32(num) => {
                let i32_type = self.context.i32_type();
                let i32_value = i32_type.const_int(*num as u64, false);
                self.builder.build_store(alloc_ptr, i32_value);
            }
            TypeValues::String(str) => {
                let str_array = self.context.i8_type();
                let i8_str = str_array.const_array(&self.str_into_array(&str));
                self.builder.build_store(alloc_ptr, i8_str);
            }
            typeofval => unimplemented!("typeof {typeofval:#?} is not supported as of right now"),
        }
    }

    fn gen_type(&self, gen_type: &Type) -> CompileResult<impl BasicType<'ctx>> {
        match &gen_type.r#type {
            Types::I8 | Types::U8 | Types::Char | Types::String => {
                let i8_type = self.context.i8_type();
                return Ok(i8_type);
            }
            Types::I32 => {
                let i32_type = self.context.i32_type();
                return Ok(i32_type);
            }
            expected_type => Err(format!(
                "expected type of {expected_type:#?} cannot be done by gen_type"
            )
            .into()),
        }
    }

    fn gen_type_array(&self, gen_type: &'ctx Type) -> CompileResult<ArrayType<'ctx>> {
        if !gen_type.is_array {
            return Err("expected type of string array to be and array".into());
        }
        match gen_type.r#type {
            Types::U8 | Types::I8 | Types::Char | Types::String => {
                let type_i8 = self.context.i8_type().array_type(gen_type.size);
                Ok(type_i8)
            }
            Types::I32 => {
                let i32_type = self.context.i32_type();
                Ok(i32_type.array_type(gen_type.size))
            }
            Types::F32 => {
                let f32_array = self.context.f32_type().array_type(gen_type.size);
                Ok(f32_array)
            }
            _ => Err("Expected array type".into()),
        }
    }

    fn gen_params(
        &self,
        params: &Vec<Paramater>,
    ) -> CompileResult<Vec<BasicMetadataTypeEnum<'ctx>>> {
        let mut meta = Vec::new();
        for param in params {
            match param.r#type.r#type {
                Types::I8 | Types::Char | Types::U8 | Types::String if param.r#type.is_array => {
                    meta.push(self.context.i8_type().array_type(param.r#type.size).into())
                }
                Types::I8 | Types::Char | Types::U8 => meta.push(self.context.i8_type().into()),
                Types::I32 => meta.push(self.context.i32_type().into()),
                Types::F32 => meta.push(self.context.f32_type().into()),
                Types::String => meta.push(
                    self.context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .into(),
                ),
                _ => todo!("Found unsuported type"),
            }
        }
        Ok(meta)
    }

    fn gen_args(&self) {
    }

    fn gen_func_call(&self, function_call: FunctionCall) {
    }
}
