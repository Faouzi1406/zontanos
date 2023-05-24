#![allow(dead_code, unused)]

pub mod gen_array;
pub mod gen_block;
pub mod gen_func_call;
pub mod gen_function;
pub mod gen_return_value;
pub mod gen_var;

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

        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module("main");

        let code_gen = CodeGen {
            builder,
            context: &context,
            module,
        };

        let compile_tree = code_gen.compile_tree(&ast);

        return Ok(compile_tree?);
    }

    pub(super) fn compile_tree(&self, ast: &'ctx Ast) -> CompileResult<String> {
        for node in ast.body.iter() {
            match node {
                Expr::Block(block) => {
                    return Err(
                        format!("Tried creating a block with no scope, is not allowed.").into(),
                    )
                }
                Expr::Logic(logic) => {}
                Expr::Variable(var) => {}
                Expr::Function(func) => {
                    let (function, block) = self.gen_function(func.clone())?;
                    self.builder.position_at_end(block);
                    self.gen_block(&func.block, Some(function));
                }
                Expr::FunctionCall(call) => {}
                Expr::Return(ret) => {}
                Expr::Program => continue,
            }
        }
        return Ok(self.module.to_string());
    }
}

impl Ast {
    /// Returns true if it starts with Expr::Program as the firt type
    fn starts_with_program(&self) -> bool {
        self.ast_type == Expr::Program
    }
}
