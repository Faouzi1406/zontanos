#![allow(dead_code, unused)]

use std::error::Error;

use super::CodeGen;
use crate::{
    ast::{block::Block, Expr},
    codegen::CompileResult,
};

type CompileBlock = Result<(), Box<dyn Error>>;

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn gen_block(&'ctx self, Block { body, line }: &'ctx Block) -> CompileBlock {
        for expr in body {
            match expr {
                Expr::Variable(var) => self.gen_var(var)?,
                Expr::FunctionCall(call) => {}
                Expr::Block(block) => {}
                Expr::Logic(logic) => {}
                Expr::Return(ret) => {}
                this => {
                    unimplemented!("This is currently not yet supported inside a block: {this:#?}")
                }
            }
        }
        Ok(())
    }
}
