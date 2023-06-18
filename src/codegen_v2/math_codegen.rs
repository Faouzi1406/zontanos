use std::collections::LinkedList;

use inkwell::values::{AnyValue, IntValue};

use crate::{
    parser_v2::ast::{Math, TypeValues},
    zon_parser::lexer::Operator,
};

use super::{CodeGen, CompileResult};

pub trait MathStatementCodegeneration<'ctx> {
    fn gen_math_value(
        &self,
        math_statement: &'ctx Math,
        current_block: Option<&str>,
    ) -> CompileResult<IntValue<'ctx>>;
}

impl<'ctx> MathStatementCodegeneration<'ctx> for CodeGen<'ctx> {
    fn gen_math_value(
        &self,
        math_statement: &'ctx Math,
        current_block: Option<&str>,
    ) -> CompileResult<IntValue<'ctx>> {
        let mut operator_stack: LinkedList<&Operator> = LinkedList::new();
        let mut num_stack: LinkedList<IntValue> = LinkedList::new();
        let mut math_statements = math_statement.0.iter().enumerate();

        while let Some((i, value)) = math_statements.next() {
            match &value.value {
                TypeValues::I32(value) => {
                    let int_type = self.context.i32_type();
                    let int_value = int_type.const_int(*value as u64, false);
                    num_stack.push_back(int_value);
                }
                TypeValues::I32Neg(neg) => {
                    operator_stack.push_back(&Operator::Min);
                    let int_type = self.context.i32_type();
                    let int_value = int_type.const_int(neg.abs() as u64, false);
                    num_stack.push_back(int_value);
                }
                TypeValues::Math(math) => {
                    let value = self.gen_math_value(math, current_block)?;
                    num_stack.push_back(value);
                }
                TypeValues::Identifier(ident) => {
                    let ident = self.get_ident(&ident, current_block)?;
                    if ident.is_int_value() {
                        let int_value = ident.into_int_value();
                        num_stack.push_back(int_value);
                        continue;
                    }

                    if ident.is_pointer_value() {
                        let load = self
                            .builder
                            .build_load(ident.into_pointer_value(), &i.to_string());

                        if load.is_int_value() {
                            num_stack.push_back(load.into_int_value());
                            continue;
                        }
                    }

                    return Err("Expected int value for Identifier".into());
                }
                TypeValues::FunctionCall(function_call, arguments) => {
                    let gen_call = self.gen_func_call(
                        function_call,
                        arguments,
                        Some(&i.to_string()),
                        current_block,
                    )?;
                    let type_value = gen_call.as_any_value_enum();

                    if type_value.is_int_value() {
                        let value = type_value.into_int_value();
                        num_stack.push_back(value);
                        continue;
                    }

                    return Err("Expected int value for Function call".into());
                }
                TypeValues::Operator(op) => match op {
                    Operator::Times => {
                        let Some(lhs) = num_stack.pop_back() else {
                           return Err("Couldn't execute the times operator considering there is no number on the left hand side of it...".into());
                       };

                        let Some((index, value)) = math_statements.next() else {
                           return Err("Couldn't execute the times operator considering there is no number on the right hand side of it...".into());
                       };

                        if let TypeValues::I32(num) = value.value {
                            let int_type = self.context.i32_type();
                            let rhs = int_type.const_int(num as u64, false);

                            let mul = self.builder.build_int_mul(lhs, rhs, &index.to_string());
                            num_stack.push_back(mul);

                            continue;
                        };

                        if let TypeValues::Math(math) = &value.value {
                            let rhs = self.gen_math_value(&math, current_block)?;
                            let mul = self.builder.build_int_mul(lhs, rhs, &index.to_string());
                            num_stack.push_back(mul);

                            continue;
                        }

                        if let TypeValues::FunctionCall(function_call, arguments) = &value.value {
                            let gen_call = self.gen_func_call(
                                function_call,
                                arguments,
                                Some(&i.to_string()),
                                current_block,
                            )?;
                            let type_value = gen_call.as_any_value_enum();

                            if type_value.is_int_value() {
                                let rhs = type_value.into_int_value();
                                let mul = self.builder.build_int_mul(lhs, rhs, &index.to_string());
                                num_stack.push_back(mul);

                                continue;
                            }
                        }

                        if let TypeValues::Identifier(ident) = &value.value {
                            let get_ident = self.get_ident(ident, current_block)?;

                            if get_ident.is_int_value() {
                                let rhs = get_ident.into_int_value();
                                let mul = self.builder.build_int_mul(lhs, rhs, &index.to_string());
                                num_stack.push_back(mul);

                                continue;
                            }

                            if get_ident.is_pointer_value() {
                                let load = self
                                    .builder
                                    .build_load(get_ident.into_pointer_value(), &i.to_string());
                                if load.is_int_value() {
                                    let rhs = load.into_int_value();
                                    let mul =
                                        self.builder.build_int_mul(lhs, rhs, &index.to_string());
                                    num_stack.push_back(mul);

                                    continue;
                                }

                                return Err("expected a int value from Identifier".into());
                            }
                        }

                        return Err("Couldn't execute the times operator considering there is no number on the right hand side of it...".into());
                    }
                    Operator::Slash => {
                        let Some(lhs) = num_stack.pop_back() else {
                           return Err("Couldn't execute the times operator considering there is no number on the left hand side of it...".into());
                       };

                        let Some((index, value)) = math_statements.next() else {
                           return Err("Couldn't execute the times operator considering there is no number on the right hand side of it...".into());
                       };

                        if let TypeValues::I32(num) = value.value {
                            let int_type = self.context.i32_type();
                            let rhs = int_type.const_int(num as u64, false);

                            let div =
                                self.builder
                                    .build_int_signed_div(lhs, rhs, &index.to_string());
                            num_stack.push_back(div);

                            continue;
                        };

                        if let TypeValues::Math(math) = &value.value {
                            let rhs = self.gen_math_value(&math, current_block)?;

                            let div =
                                self.builder
                                    .build_int_signed_div(lhs, rhs, &index.to_string());
                            num_stack.push_back(div);

                            continue;
                        }

                        return Err("Couldn't execute the times operator considering there is no number on the right hand side of it...".into());
                    }
                    Operator::Plus => {
                        operator_stack.push_back(op);
                    }
                    Operator::Min => {
                        operator_stack.push_back(op);
                    }
                    _ => unreachable!(
                        "to invalid statements, this error should get caught by the parser"
                    ),
                },
                _ => unreachable!(
                    "to invalid statements, this error should get caught by the parser"
                ),
            }
        }

        let mut operator_iter = operator_stack.iter().enumerate();
        while let Some((index, operator)) = operator_iter.next() {
            match operator {
                Operator::Plus => {
                    let Some(lhs) = num_stack.pop_front() else {
                        return Err("No number on the left hand side of plus operator".into());
                    };

                    let Some(rhs) = num_stack.pop_front() else {
                        return Err("No number on the right hand side of plus operator".into());
                    };

                    let value = self.builder.build_int_add(lhs, rhs, &index.to_string());
                    num_stack.push_front(value);
                }
                Operator::Min => {
                    let Some(lhs) = num_stack.pop_front() else {
                        return Err("No number on the left hand side of plus operator".into());
                    };

                    let Some(rhs) = num_stack.pop_front() else {
                        return Err("No number on the right hand side of plus operator".into());
                    };

                    let value = self.builder.build_int_sub(lhs, rhs, &index.to_string());
                    num_stack.push_front(value);
                }
                _ => unreachable!("invalid operator on operator stack"),
            }
        }

        let Some(pop_last_value) = num_stack.pop_front() else {
            return Err("no math value on stack...".into())
        };

        Ok(pop_last_value)
    }
}
