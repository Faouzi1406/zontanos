#![allow(dead_code)]

use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{ArrayType, BasicMetadataTypeEnum, BasicType};
use inkwell::values::{AnyValue, BasicMetadataValueEnum, CallSiteValue, IntValue, PointerValue};
use inkwell::AddressSpace;
use inkwell::{builder::Builder, values::FunctionValue};
use std::error::Error;

use crate::parser_v2::ast::{
    Ast, Function, FunctionCall, Node, NodeTypes, Paramater, Type, TypeValues, Types, Value,
    Variable,
};

pub struct CodeGen<'ctx> {
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub context: &'ctx Context,
    pub scope: Option<(FunctionValue<'ctx>, &'ctx Function)>,
}

type CompileResult<T> = Result<T, Box<dyn Error>>;

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_ast(&mut self, ast: &'ctx Ast) -> CompileResult<()> {
        for node in &ast.body {
            match &node.node_type {
                NodeTypes::Function(func) => {
                    let function = self.gen_func(func)?;
                    let _block = self.gen_block(function, &func.body)?;
                    self.scope = Some((function, func));
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
                    if let NodeTypes::Value(value) = &node.right.as_ref().unwrap().node_type {
                        self.gen_alloca_store(var, value)?;
                    };

                    if let NodeTypes::FunctionCall(call) = &node.right.as_ref().unwrap().node_type {
                        let Some(arguments) = call.get_args(&node.right.as_ref().unwrap()) else { 
                            panic!("the right node of the function call did not contain any arguments.")
                        };
                        // todo: type check for function call
                        let _ = self.gen_func_call(call, arguments, Some(&var.ident.name))?;
                    }
                }
                NodeTypes::FunctionCall(call) => {
                    let Some(args) = call.get_args(node) else {return Err("expected the arguments of a function to be in the left branch".into())};
                    self.gen_func_call(call, &args, None)?;
                }
                NodeTypes::Return => {
                    let _ = self.gen_return(node);
                }
                node_type => unimplemented!("Support for {node_type:#?} in blocks is not yet implemented. found this value on line: {}", node.line)
            }
        }
        Ok(block)
    }

    fn gen_return(&self, return_node: &'ctx Node) -> CompileResult<()> {
        let Some(value_node) = &return_node.right else { panic!("Expected right node type of return node to have a value") };
        let NodeTypes::Value(value) =  &value_node.node_type else { panic!("Expected right node type of return node to have a value") };
        match &value.value {
            TypeValues::I8(num) => {
                let i8_type = self.context.i8_type();
                let i8_value = i8_type.const_int(*num as u64, false);
                self.builder.build_return(Some(&i8_value));
                return Ok(());
            }
            TypeValues::U8(num) => {
                let i8_type = self.context.i8_type();
                let i8_value = i8_type.const_int(*num as u64, false);
                self.builder.build_return(Some(&i8_value));
                return Ok(());
            }
            TypeValues::I32(num) => {
                let i32_type = self.context.i32_type();
                let i32_value = i32_type.const_int(*num as u64, false);
                self.builder.build_return(Some(&i32_value));
                return Ok(());
            }
            TypeValues::String(str) => {
                let str_array = self.context.i8_type();
                let i8_str = str_array.const_array(&self.str_into_array(&str));
                self.builder.build_return(Some(&i8_str));
                return Ok(());
            }
            TypeValues::FunctionCall(call) => {
                let Some(arguments) = call.get_args(&value_node) else { panic!("Expected function call node to have arguments") };
                if let Some(func) = self.module.get_function(&call.calls_to.name) {
                    let args = self.gen_args(arguments);
                    let call: CallSiteValue<'ctx> = self.builder.build_call(func, &args, "return");
                    let call_type = call.as_any_value_enum();

                    // Todo: Add check for the function return type and the calls return type |
                    // give compile error if not equal
                    if call_type.is_int_value() {
                        let call = call_type.into_int_value();
                        self.builder.build_return(Some(&call));
                    }

                    if call_type.is_array_value() {
                        let call = call_type.into_array_value();
                        self.builder.build_return(Some(&call));
                    }

                    if call_type.is_float_value() {
                        let call = call_type.into_array_value();
                        self.builder.build_return(Some(&call));
                    }

                    if call_type.is_pointer_value() {
                        let call = call_type.into_array_value();
                        self.builder.build_return(Some(&call));
                    }

                    return Ok(());
                }
                return Err(
                    format!("Couldn't find any function named: {}", call.calls_to.name).into(),
                );
            }
            typeofval => unimplemented!("typeof {typeofval:#?} is not supported as of right now"),
        }
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

    fn gen_args(&self, arguments: &'ctx Vec<Value>) -> Vec<BasicMetadataValueEnum<'ctx>> {
        let mut args = Vec::new();
        for arg in arguments {
            match &arg.value {
                TypeValues::I8(i8_value) => {
                    let i8_type = self.context.i8_type();
                    let value = i8_type.const_int(*i8_value as u64, false);
                    args.push(value.into());
                }
                TypeValues::U8(u8_value) => {
                    let u8_type = self.context.i8_type();
                    let value = u8_type.const_int(*u8_value as u64, false);
                    args.push(value.into());
                }
                TypeValues::I32(i32_value) => {
                    let i32_type = self.context.i32_type();
                    let value = i32_type.const_int(*i32_value as u64, false);
                    args.push(value.into());
                }
                TypeValues::F32(f32_value) => {
                    let f32_type = self.context.f32_type();
                    let value = f32_type.const_float(*f32_value as f64);
                    args.push(value.into());
                }
                TypeValues::Char(char_value) => {
                    let i8_type = self.context.i8_type();
                    let value = i8_type.const_int(*char_value as u64, false);
                    args.push(value.into());
                }
                TypeValues::String(str) => {
                    let i8_type = self.context.i8_type();
                    let str_array = self.str_into_array(str);
                    let value = i8_type.const_array(&str_array);
                    args.push(value.into());
                }
                value => unimplemented!("support for type of {value:#?}"),
            }
        }
        args
    }

    fn gen_func_call(
        &self,
        function_call: &'ctx FunctionCall,
        arguments: &'ctx Vec<Value>,
        call_name: Option<&str>
    ) -> CompileResult<CallSiteValue<'ctx>> {
        if let Some(called_func) = self.module.get_function(&function_call.calls_to.name) {
            let args = &self.gen_args(arguments);
            let call_name = call_name.unwrap_or("call");
            let value = self.builder.build_call(called_func, args, call_name);
            Ok(value)
        } else {
            Err(format!(
                "Couldnt' find any function named: {}",
                function_call.calls_to.name
            )
            .into())
        }
    }
}
