#![allow(dead_code, unused)]

use inkwell::{
    types::ArrayType,
    values::{BasicValue, IntValue},
};
use std::{borrow::Borrow, error::Error, todo};

use super::CodeGen;
use crate::{
    ast::{
        block::Block,
        r#return::Return,
        types::{MarkerTypes, VarTypes},
        Expr,
    },
    codegen::CompileResult,
};

type CompileBlock = Result<(), Box<dyn Error>>;

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn gen_block(&self, Block { body, line }: &'ctx Block) -> CompileBlock {
        for expr in body {
            match expr {
                Expr::Variable(var) => self.gen_scoped_var(&var)?,
                Expr::FunctionCall(call) => {}
                Expr::Block(block) => {}
                Expr::Logic(logic) => {}
                Expr::Return(ret) => {
                    if ret.is_int_return() {
                        if let Some(int_return) = self.get_int_return_value(ret) {
                            self.builder.build_return(Some(&int_return));
                        };
                        continue;
                    }
                    if ret.is_float_return() {
                        if let Some(int_return) = self.gen_float_return_type(ret) {
                            self.builder.build_return(Some(&int_return));
                        };
                        continue;
                    }
                    if ret.is_array_return() {
                        if let Some(int_return) = self.gen_arr_return_type(ret) {
                            self.builder.build_return(Some(&int_return));
                        };
                        continue;
                    }
                }
                this => {
                    unimplemented!("This is currently not yet supported inside a block: {this:#?}")
                }
            }
        }
        Ok(())
    }
}
