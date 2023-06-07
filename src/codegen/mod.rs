#![allow(dead_code, unused)]

pub mod gen_array;
pub mod gen_block;
pub mod gen_func_call;
pub mod gen_function;
pub mod gen_return_value;
pub mod gen_var;
pub mod std_gen;

use crate::ast::{block::Block, Ast, Expr};
use inkwell::{builder::Builder, context::Context, module::Module};
use std::error::Error;

type CompileResult<T> = Result<T, Box<dyn Error>>;

pub struct CodeGen<'ctx> {
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub context: &'ctx Context,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_default(ast: &'ctx Ast) -> CompileResult<String> {
        if !ast.starts_with_program() {
            return Err("The first node in the AST wasn't a program node".into());
        }
        return Ok(Self::compile_tree(ast)?);
    }

    pub(super) fn compile_tree(ast: &'ctx Ast) -> CompileResult<String> {
        let context = Context::create();
        let module = context.create_module("main");

        let code_gen = CodeGen {
            builder: context.create_builder(),
            context: &context,
            module,
        };

        for node in ast.body.iter() {
            match node {
                Expr::Block(block) => {
                    return Err("Tried creating a block with no scope, is not allowed.".into())
                }
                Expr::Logic(logic) => {}
                Expr::Variable(var) => {}
                Expr::Function(func) => {
                    let (function, block) = code_gen.gen_function(&func)?;
                    code_gen.builder.position_at_end(block);
                    code_gen.gen_block(&func.block, Some(function), Some(func))?;
                }
                Expr::FunctionCall(call) => {}
                Expr::Return(ret) => {
                    return Err(
                        format!("You can not have return statemetns outside of functions").into(),
                    )
                }
                Expr::Program => continue,
            }
        }

        return Ok(code_gen.module.to_string());
    }
}

impl Ast {
    /// Returns true if it starts with Expr::Program as the firt type
    fn starts_with_program(&self) -> bool {
        self.ast_type == Expr::Program
    }
}
