use std::collections::LinkedList;

use inkwell::values::IntValue;

use crate::{
    parser_v2::ast::{Math, TypeValues},
    zon_parser::lexer::{Operator, Tokens},
};

use super::{CodeGen, CompileResult};

pub trait MathStatementCodegeneration<'ctx> {
    fn gen_math_value(&self, math_statement: &'ctx Math) -> CompileResult<IntValue<'ctx>>;
}

impl<'ctx> MathStatementCodegeneration<'ctx> for CodeGen<'ctx> {
    fn gen_math_value(&self, math_statement: &'ctx Math) -> CompileResult<IntValue<'ctx>> {
        let mut operator_stack: LinkedList<&Operator> = LinkedList::new();
        let mut num_stack: LinkedList<IntValue> = LinkedList::new();
        let mut math_statements = math_statement.0.iter().enumerate();

        while let Some((_, value)) = math_statements.next() {
            match &value {
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
                    let value = self.gen_math_value(math)?;
                    num_stack.push_back(value);
                }
                TypeValues::Operator(op) => match op {
                    Operator::Times => {
                        let Some(lhs) = num_stack.pop_back() else {
                           return Err("Couldn't execute the times operator considering there is no number on the left hand side of it...".into());
                       };

                        let Some((index, value)) = math_statements.next() else {
                           return Err("Couldn't execute the times operator considering there is no number on the right hand side of it...".into());
                       };

                        if let TypeValues::I32(num) = value {
                            let int_type = self.context.i32_type();
                            let rhs = int_type.const_int(*num as u64, false);

                            let mul = self.builder.build_int_mul(lhs, rhs, &index.to_string());
                            num_stack.push_back(mul);

                            continue;
                        };

                        if let TypeValues::Math(math) = value {
                            let rhs = self.gen_math_value(math)?;
                            let mul = self.builder.build_int_mul(lhs, rhs, &index.to_string());
                            num_stack.push_back(mul);

                            continue;
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

                        if let TypeValues::I32(num) = value {
                            let int_type = self.context.i32_type();
                            let rhs = int_type.const_int(*num as u64, false);

                            let div = self.builder.build_int_signed_div(lhs, rhs, &index.to_string());
                            num_stack.push_back(div);

                            continue;
                        };

                        if let TypeValues::Math(math) = value {
                            let rhs = self.gen_math_value(math)?;

                            let div = self.builder.build_int_signed_div(lhs, rhs, &index.to_string());
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
