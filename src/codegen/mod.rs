#![allow(dead_code, unused)]

pub mod gen_array;
pub mod gen_block;
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
    pub fn compile_default(&'ctx self, ast: Ast) -> CompileResult<String> {
        if !ast.starts_with_program() {
            return Err("The first node in the AST wasn't a program node".into());
        }

        let compile_tree = self.compile_tree(&ast);

        return Ok(compile_tree?);
    }

    pub(super) fn compile_tree(&'ctx self, ast:  Ast) -> CompileResult<String> {
        for node in &ast.body {
            match node {
                Expr::Block(block) => self.gen_block(block)?,
                Expr::Logic(logic) => {}
                Expr::Variable(var) => {}
                Expr::Function(func) => {}
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
