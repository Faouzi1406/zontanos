#![allow(dead_code)]

pub mod zonc;
mod math_codegen;
mod lep_codegen;

use inkwell::values::{PointerValue, BasicValueEnum};
use crate::parser_v2::ast::Assignment;
use crate::zon_parser::lexer::Operator;
use inkwell::basic_block::BasicBlock;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{AnyTypeEnum, ArrayType, BasicMetadataTypeEnum, BasicType};
use inkwell::values::{
    AnyValue, AnyValueEnum, ArrayValue, BasicMetadataValueEnum, CallSiteValue, IntValue,
};
use inkwell::AddressSpace;
use inkwell::{builder::Builder, values::FunctionValue};
use std::error::Error;
use crate::parser_v2::ast::{
    Ast, Function, FunctionCall, Node, NodeTypes, Paramater, Type, TypeValues, Types, Value,
    Variable,
};

use self::math_codegen::MathStatementCodegeneration;
use self::zonc::GenC;

pub struct CodeGen<'ctx> {
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub context: &'ctx Context,
    pub scope: Option<(FunctionValue<'ctx>, &'ctx Function)>,
}

pub(super) type CompileResult<T> = Result<T, Box<dyn Error>>;

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_ast(&mut self, ast: &'ctx Ast) -> CompileResult<()> {
        for node in &ast.body {
            match &node.node_type {
                NodeTypes::Function(func) => {
                    let function = self.gen_func(func)?;
                    self.scope = Some((function, func));
                    let _block = self.gen_block(function, &func.body, Some("entry"))?;
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
            let params = self.gen_params(&func.paramaters)?;
            if let Ok(return_type) = self.gen_type(&func.returns) {
                let return_type = return_type.fn_type(&params, false);
                return Ok(self
                    .module
                    .add_function(&func.ident.name, return_type, None));
            }
            Ok(self.module.add_function(
                &func.ident.name,
                self.context.void_type().fn_type(&params, false),
                None,
            ))
        }
    }

    fn gen_block(
        &self,
        func: FunctionValue<'ctx>,
        nodes: &'ctx Vec<Node>,
        block_name: Option<&str>,
    ) -> CompileResult<BasicBlock<'ctx>> {
        let block_name = block_name.unwrap_or("entry");
        let block = self.context.append_basic_block(func, block_name);
        self.builder.position_at_end(block);

        for node in nodes {
            match &node.node_type {
                NodeTypes::Variable(var) => {
                    if let NodeTypes::Value(value) = &node.right.as_ref().unwrap().node_type {
                        self.gen_alloca_store(var, value, Some(block_name))?;
                    };

                    if let NodeTypes::FunctionCall(call) = &node.right.as_ref().unwrap().node_type {
                        let Some(arguments) = call.get_args(&node.right.as_ref().unwrap()) else { 
                            panic!("the right node of the function call did not contain any arguments.")
                        };
                        // todo: type check for function call
                        let _ = self.gen_func_call(call, arguments, Some(&var.ident.name), Some(block_name))?;
                    }
                }
                NodeTypes::FunctionCall(call) => {
                    let Some(args) = call.get_args(node) else {return Err("expected the arguments of a function to be in the left branch".into())};
                    self.gen_func_call(call, &args, None, Some(block_name))?;
                }
                NodeTypes::LogicalStatement(statement) => {
                    self.gen_logcal_statement(statement, Some(block_name))?;
                }
                NodeTypes::Assignment(assignment) => {
                    self.gen_reassignment(assignment, node, Some(block_name))?;
                }
                NodeTypes::Return => {
                    let _ = self.gen_return(node, Some(block_name));
                }
                node_type => unimplemented!("Support for {node_type:#?} in blocks is not yet implemented. found this value on line: {}", node.line)
            }
        }
        Ok(block)
    }
    fn gen_reassignment(&self, assignment: &'ctx Assignment, node: &'ctx Node, block_name: Option<&str>) -> CompileResult<()> {
        let get_ident = self.get_ident(&assignment.assigns_to.name, block_name)?;
        if get_ident.is_pointer_value() {
            let Some(op) = assignment.get_op(node) else { return Err("expected a operator for gen_reassignment".into()) };
            let Some(value) = assignment.get_value(node) else { return Err("expected a value for gen_reassignment".into()) };

            match op {
                Operator::Eq => {
                    let ptr = get_ident.into_pointer_value();  
                    self.gen_store(value, ptr, None, block_name);
                }
                Operator::PlusIs => {
                    let ptr = get_ident.into_pointer_value();  
                    let load = self.builder.build_load(ptr, "load_val");
                    match load {
                        BasicValueEnum::IntValue(int_value) => {
                            if let TypeValues::I32(value) = value.value {
                                let int = self.context.i32_type();
                                let add_value = int.const_int(value as u64, false);
                                let add = self.builder.build_int_add(int_value, add_value, "add_op");
                                self.builder.build_store(ptr, add);
                            };
                            if let TypeValues::Identifier(ident) = &value.value {
                                let get_ident = self.get_ident(&ident, block_name)?;

                                if get_ident.is_pointer_value() {
                                    let load = self.builder.build_load(get_ident.into_pointer_value(), "load_other");
                                    match load {
                                        BasicValueEnum::IntValue(add_value) => {
                                            let add = self.builder.build_int_add(int_value, add_value, "add_op");
                                            self.builder.build_store(ptr, add);
                                        }
                                        _ => return Err("Expected a integer for TimesIs operator".into())
                                    }
                                } 
                                else if get_ident.is_int_value() {
                                    let add_value = get_ident.into_int_value(); 
                                    let add = self.builder.build_int_add(int_value, add_value, "add_op");
                                    self.builder.build_store(ptr, add);
                                }
                            }
                            if let TypeValues::FunctionCall(call, arguments) = &value.value {
                                let call = self.gen_func_call(call, arguments, Some("tiscall"), block_name)?;
                                let value = call.as_any_value_enum();

                                match value {
                                    AnyValueEnum::IntValue(add_value) => {
                                        let add = self.builder.build_int_add(int_value, add_value, "add_op");
                                        self.builder.build_store(ptr, add);
                                    }
                                    _ => return Err("Expected a integer for TimesIs operator".into())
                                }
                            }
                        }
                        _ => return Err("Expected a integer for additation operator".into())
                    }
                }
                Operator::TimesIs => {
                    let ptr = get_ident.into_pointer_value();  
                    let load = self.builder.build_load(ptr, "load_val");
                    match load {
                        BasicValueEnum::IntValue(int_value) => {
                            if let TypeValues::I32(value) = value.value {
                                let int = self.context.i32_type();
                                let add_value = int.const_int(value as u64, false);
                                let add = self.builder.build_int_mul(int_value, add_value, "times_op");
                                self.builder.build_store(ptr, add);
                            };
                            if let TypeValues::Identifier(ident) = &value.value {
                                let get_ident = self.get_ident(&ident, block_name)?;

                                if get_ident.is_pointer_value() {
                                    let load = self.builder.build_load(get_ident.into_pointer_value(), "load_other");
                                    match load {
                                        BasicValueEnum::IntValue(add_value) => {
                                            let add = self.builder.build_int_mul(int_value, add_value, "times_op");
                                            self.builder.build_store(ptr, add);
                                        }
                                        _ => return Err("Expected a integer for TimesIs operator".into())
                                    }
                                } 
                                else if get_ident.is_int_value() {
                                    let add_value = get_ident.into_int_value(); 
                                    let add = self.builder.build_int_mul(int_value, add_value, "times_op");
                                    self.builder.build_store(ptr, add);
                                }
                            }
                            if let TypeValues::FunctionCall(call, arguments) = &value.value {
                                let call = self.gen_func_call(call, arguments, Some("tiscall"), block_name)?;
                                let value = call.as_any_value_enum();

                                match value {
                                    AnyValueEnum::IntValue(add_value) => {
                                        let add = self.builder.build_int_mul(int_value, add_value, "times_op");
                                        self.builder.build_store(ptr, add);
                                    }
                                    _ => return Err("Expected a integer for TimesIs operator".into())
                                }
                            }
                        }
                        _ => return Err("Expected a integer for TimesIs operator".into())
                    }
                }
                Operator::MinusIs => {
                    let ptr = get_ident.into_pointer_value();  
                    let load = self.builder.build_load(ptr, "load_val");
                    match load {
                        BasicValueEnum::IntValue(int_value) => {
                            if let TypeValues::I32(value) = value.value {
                                let int = self.context.i32_type();
                                let add_value = int.const_int(value as u64, false);
                                let add = self.builder.build_int_sub(int_value, add_value, "minus_op");
                                self.builder.build_store(ptr, add);
                            };
                            if let TypeValues::Identifier(ident) = &value.value {
                                let get_ident = self.get_ident(&ident, block_name)?;

                                if get_ident.is_pointer_value() {
                                    let load = self.builder.build_load(get_ident.into_pointer_value(), "load_other");
                                    match load {
                                        BasicValueEnum::IntValue(add_value) => {
                                            let add = self.builder.build_int_sub(int_value, add_value, "minus_op");
                                            self.builder.build_store(ptr, add);
                                        }
                                        _ => return Err("Expected a integer for TimesIs operator".into())
                                    }
                                } 
                                else if get_ident.is_int_value() {
                                    let add_value = get_ident.into_int_value(); 
                                    let add = self.builder.build_int_sub(int_value, add_value, "minus_op");
                                    self.builder.build_store(ptr, add);
                                }
                            }
                            if let TypeValues::FunctionCall(call, arguments) = &value.value {
                                let call = self.gen_func_call(call, arguments, Some("tiscall"), block_name)?;
                                let value = call.as_any_value_enum();

                                match value {
                                    AnyValueEnum::IntValue(add_value) => {
                                        let add = self.builder.build_int_sub(int_value, add_value, "minus_op");
                                        self.builder.build_store(ptr, add);
                                    }
                                    _ => return Err("Expected a integer for TimesIs operator".into())
                                }
                            }
                        }
                        _ => return Err("Expected a integer for MinusIs operator".into())
                    }
                }
                _ => return Err(format!("the operator {:#?} is not a valid reassignment operator.", op).into())
            }
        }
        Ok(())
    }

    fn gen_return(&self, return_node: &'ctx Node, block_name: Option<&str>) -> CompileResult<()> {
        let Some(value_node) = &return_node.right else { panic!("Expected right node type of return node to have a value") };
        if let NodeTypes::Value(value) = &value_node.node_type {
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
                TypeValues::Char(char) => {
                    let i8_type = self.context.i8_type();
                    let i8_value = i8_type.const_int(*char as u64, false);
                    self.builder.build_return(Some(&i8_value));
                    return Ok(());
                }
                TypeValues::I32(num) => {
                    let i32_type = self.context.i32_type();
                    let i32_value = i32_type.const_int(*num as u64, false);
                    self.builder.build_return(Some(&i32_value));
                    return Ok(());
                }
                TypeValues::Array(array) => {
                    let Some((_,function)) = self.scope else {
                        return Err("Expected the value of and return array to be in a function".into());
                    };
                    let function_returns = &function.returns;
                    let arr = self.gen_array_values(array, function_returns);
                    self.builder.build_return(Some(&arr));
                    return Ok(());
                }
                TypeValues::Math(math) => {
                    let math = self.gen_math_value(math, block_name)?;
                    self.builder.build_return(Some(&math));
                    return Ok(())
                }
                TypeValues::Identifier(ident) => {
                    let ident = self.get_ident(&ident, block_name)?;
                    let value = ident.as_any_value_enum();
                    match value.get_type() {
                        AnyTypeEnum::IntType(_) => {
                            self.builder.build_return(Some(&value.into_int_value()));
                        }
                        AnyTypeEnum::ArrayType(_) => {
                            self.builder.build_return(Some(&value.into_array_value()));
                        }
                        AnyTypeEnum::VoidType(_) => {
                            self.builder.build_return(None);
                        }
                        AnyTypeEnum::PointerType(_) => {
                            let load = self.builder.build_load(ident.into_pointer_value(), "ret_load");
                            self.builder.build_return(Some(&load));
                        }
                        typeof_return => unimplemented!(
                            "Typeof {typeof_return} can not be used for returning values"
                        ),
                    }
                }
                TypeValues::String(str) => {
                    let str_array = self.context.i8_type();
                    let i8_str = str_array.const_array(&self.str_into_array(&str));
                    self.builder.build_return(Some(&i8_str));
                    return Ok(());
                }
                TypeValues::None => {
                    self.builder.build_return(None);
                    return Ok(());
                }
                typeofval => {
                    unimplemented!("typeof {typeofval:#?} is not supported as of right now")
                }
            }
        }
        if let NodeTypes::FunctionCall(call) = &value_node.node_type {
            let Some(arguments) = call.get_args(&value_node) else { panic!("Expected function call node to have arguments") };
            if let Some(func) = self.module.get_function(&call.calls_to.name) {
                let args = self.gen_args(arguments, block_name)?;
                let call: CallSiteValue<'ctx> = self.builder.build_call(func, &args, "return");
                let call_type = call.as_any_value_enum();

                // Todo: Add check for the function return type and the calls return type |
                // give compile error if not equal
                if call_type.is_int_value() {
                    let call = call_type.into_int_value();
                    self.builder.build_return(Some(&call));
                    return Ok(())
                }

                if call_type.is_array_value() {
                    let call = call_type.into_array_value();
                    self.builder.build_return(Some(&call));
                    return Ok(())
                }

                if call_type.is_float_value() {
                    let call = call_type.into_array_value();
                    self.builder.build_return(Some(&call));
                    return Ok(())
                }

                if call_type.is_pointer_value() {
                    let call = call_type.into_array_value();
                    self.builder.build_return(Some(&call));
                    return Ok(());
                }

                self.builder.build_return(None);
                return Ok(())
            }
            return Err(format!("Couldn't find any function named: {}", call.calls_to.name).into());
        }
        Ok(())
    }

    fn gen_array_values(&self, array_values: &'ctx Vec<TypeValues>, type_of: &Type) -> ArrayValue {
        let mut values: Vec<IntValue> = Vec::new();
        for value in array_values {
            match value {
                TypeValues::Char(char_value) => {
                    let char_type = self.context.i8_type();
                    let char_value = char_type.const_int(*char_value as u64, false);
                    values.push(char_value)
                }
                TypeValues::I8(num) => {
                    let i8_type = self.context.i8_type();
                    let i8_value = i8_type.const_int(*num as u64, false);
                    values.push(i8_value.into())
                }
                TypeValues::U8(num) => {
                    let i8_type = self.context.i8_type();
                    let i8_value = i8_type.const_int(*num as u64, false);
                    values.push(i8_value.into())
                }
                TypeValues::I32(num) => {
                    let i32_type = self.context.i32_type();
                    let i32_value = i32_type.const_int(*num as u64, false);
                    values.push(i32_value.into())
                }
                typeofval => {
                    unimplemented!("typeof {typeofval:#?} is not supported as of right now")
                }
            }
        }

        match &type_of.r#type {
            Types::I8 | Types::Char | Types::U8 => {
                let i8_type = self.context.i8_type();
                let array = i8_type.const_array(&values);
                return array;
            }
            Types::I32 => {
                let i8_type = self.context.i32_type();
                let array = i8_type.const_array(&values);
                return array;
            }
            typeofval => unimplemented!("typeof {typeofval:#?} is not supported as of right now"),
        }
    }

    fn gen_alloca_store(&self, variable: &'ctx Variable, value: &'ctx Value, block_name: Option<&str>) -> CompileResult<()> {
        if variable.var_type.is_array {
            let arr_type = self.gen_type_array(&variable.var_type)?;
            let alloc = self.builder.build_alloca(arr_type, &variable.ident.name);
            self.gen_store(value, alloc, Some(&variable.var_type), block_name);
            Ok(())
        } else {
            let var_type = self.gen_type(&variable.var_type)?;
            let alloc = self.builder.build_alloca(var_type, &variable.ident.name);
            self.gen_store(value, alloc, Some(&variable.var_type), block_name);
            Ok(())
        }
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

    fn gen_store(&self, value: &'ctx Value, alloc_ptr: PointerValue, type_of: Option<&'ctx Type>, block_name: Option<&str>) {
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
            TypeValues::I32Neg(value) => {
                let i32_type = self.context.i32_type();
                let i32_value = i32_type.const_int(*value as u64, false);
                self.builder.build_store(alloc_ptr, i32_value);
            }
            TypeValues::String(str) => {
                let str_array = self.context.i8_type();
                let i8_str = str_array.const_array(&self.str_into_array(&str));
                self.builder.build_store(alloc_ptr, i8_str);
            }
            TypeValues::Char(char) => {
                let str_array = self.context.i8_type();
                let i8_str = str_array.const_int(*char as u64, false);
                self.builder.build_store(alloc_ptr, i8_str);
            }
            TypeValues::Array(arr) => {
                let type_of = type_of.unwrap();
                let array = self.gen_array_values(&arr, type_of);
                self.builder.build_store(alloc_ptr, array);
            }
            TypeValues::Math(math) => {
                // TODO: Fix error here, don't just expect 
                let value = self.gen_math_value(math, block_name).expect("couldn't parse math statement");
                self.builder.build_store(alloc_ptr, value);
            }
            TypeValues::Identifier(ident) => {
                let Ok(ident) = self.get_ident(ident, block_name) else {
                    panic!("temp: no such ident: {ident}");
                };
                if ident.is_pointer_value() {
                    let load = self.builder.build_load(ident.into_pointer_value(), "load_for_store");
                    self.builder.build_store(alloc_ptr, load);
                }
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
                Types::I8 | Types::Char | Types::U8 if param.r#type.is_pointer => {
                    meta.push(self.context.i8_type().ptr_type(Default::default()).into())
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

    fn gen_args(
        &self,
        arguments: &'ctx Vec<Value>,
        block_name: Option<&str>
    ) -> CompileResult<Vec<BasicMetadataValueEnum<'ctx>>> {
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
                TypeValues::String(str) if arg.is_ptr => {
                    let ptr = self.builder.build_global_string_ptr(str, "str_pointer");
                    args.push(ptr.as_pointer_value().into());
                }
                TypeValues::String(str) => {
                    let i8_type = self.context.i8_type();
                    let str_array = self.str_into_array(str);
                    let value = i8_type.const_array(&str_array);
                    args.push(value.into());
                }
                TypeValues::FunctionCall(calls, arguments) => {
                    let call = self.gen_func_call(calls, arguments, None, block_name)?;
                    let call_type = call.as_any_value_enum();

                    // Todo: Add check for the function return type and the calls return type |
                    // give compile error if not equal
                    if call_type.is_int_value() {
                        let call = call_type.into_int_value();
                        args.push(call.into());
                    }

                    if call_type.is_array_value() {
                        let call = call_type.into_array_value();
                        args.push(call.into());
                    }

                    if call_type.is_pointer_value() {
                        let call = call_type.into_array_value();
                        args.push(call.into());
                    }
                }
                TypeValues::Math(math) => {
                    let math_value = self.gen_math_value(math, block_name)?;
                    args.push(math_value.into());
                }
                TypeValues::Identifier(ident) if arg.is_ptr => {
                    // TODO: FIX THIS DON'T CONTINUE RETURN ERR, THIS IS JUST FOR NOW, OKAY
                    let Ok(value) = self.get_ident(&ident, block_name) else { continue };
                    args.push(value.into_pointer_value().into());
                }
                TypeValues::Identifier(ident) => {
                    // TODO: FIX THIS DON'T CONTINUE RETURN ERR, THIS IS JUST FOR NOW, OKAY
                    let Ok(value) = self.get_ident(&ident, block_name) else { continue };
                    if value.is_pointer_value() {
                        let load_value = self
                            .builder
                            .build_load(value.into_pointer_value(), "loaded");

                        args.push(load_value.into());
                        continue;
                    } else {
                        args.push(value.into());
                    }
                }
                value => unimplemented!("support for type of {value:#?}"),
            }
        }
        Ok(args)
    }

    fn gen_func_call(
        &self,
        function_call: &'ctx FunctionCall,
        arguments: &'ctx Vec<Value>,
        call_name: Option<&str>,
        block_name: Option<&str>
    ) -> CompileResult<CallSiteValue<'ctx>> {
        if let Some(called_func) = self.module.get_function(&function_call.calls_to.name) {
            let args = &self.gen_args(arguments, block_name)?;
            let call_name = call_name.unwrap_or("call");
            let value = self.builder.build_call(called_func, args, call_name);
            return Ok(value);
        }

        if let Some(c_func) = self.gen_c_function(&function_call.calls_to.name) {
            let args = &self.gen_args(arguments, block_name)?;
            let call_name = call_name.unwrap_or("call");
            let value = self.builder.build_call(c_func, args, call_name);
            return Ok(value);
        }

        Err(format!(
            "Couldnt' find any function named: {}",
            function_call.calls_to.name
        )
        .into())
    }
}

impl<'ctx> CodeGen<'ctx> {
    fn get_ident(&self, name: &str, current_block: Option<&str>) -> CompileResult<BasicMetadataValueEnum<'ctx>> {
        let Some((function, function_node)) = self.scope else {
            return Err(format!("Tried to get value with the name {}, but the current scope is none", name).into());
        };

        let blocks = function.get_basic_blocks();
        let Some(current_block) = blocks.iter().find(|x| x.get_name().to_str() == Ok(current_block.unwrap_or("entry"))) else {
            return Err(format!("Tried to get value with the name {}, but the current scope is none", name).into());
        };


        if let Some(param) = function_node.get_param_index_with_name(&name) {
            let Some(param) = function.get_nth_param(param as u32) else {
                return Err(format!("Tried to get value with the name {}, but the current scope is none", name).into());
            };
            return Ok(param.into());
        };

        let Some(ident) = current_block.get_instruction_with_name(&name) else {
            return Err(format!("There is no variable called {}", name).into());
        };

        match ident.as_any_value_enum() {
            AnyValueEnum::IntValue(value) => Ok(value.into()),
            AnyValueEnum::ArrayValue(array) => Ok(array.into()),
            AnyValueEnum::PointerValue(value) => Ok(value.into()),
            _ => unimplemented!("Found unimplemented value, this should not be possible."),
        }
    }
}
