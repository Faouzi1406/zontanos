#![allow(dead_code, unused)]

use inkwell::{
    types::ArrayType,
    values::{BasicValue, FunctionValue, IntValue},
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
    pub(super) fn gen_block(
        &self,
        Block { body, line }: &'ctx Block,
        scope: Option<FunctionValue>,
    ) -> CompileBlock {
        for expr in body {
            match expr {
                Expr::Variable(var) => {
                    let Some(scope) = scope else {
                        return Err("tried to creat a scoped variable but no scope was given".into());
                    };
                    self.gen_scoped_var(&var, scope)?
                }
                Expr::FunctionCall(call) => {}
                Expr::Block(block) => {}
                Expr::Logic(logic) => {}
                Expr::Return(ret) => self.gen_function_return(ret),
                this => {
                    unimplemented!("This is currently not yet supported inside a block: {this:#?}")
                }
            }
        }
        Ok(())
    }
}
