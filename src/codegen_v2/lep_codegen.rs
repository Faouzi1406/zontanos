use inkwell::{IntPredicate, values::{IntMathValue, IntValue}};

use super::{CodeGen, CompileResult};
use crate::parser_v2::{
    ast::{TypeValues, Value, Types},
    parser::lep::{LogicalStatement, Statements},
};

impl<'ctx> CodeGen<'ctx> {
    fn gen_logcal_statement(&self, logical_statement: &Box<LogicalStatement>) {}

    fn gen_case(&self, statements: &Vec<Statements>) {
        for statement in statements {
            match statement {
                _ => todo!(),
            }
        }
    }

    fn gen_or_case(&self, value: Value, or: Value, operator: IntPredicate) -> CompileResult<IntValue> {
        match (value.value, or.value) {
            (TypeValues::I32(value), TypeValues::I32(other)) => {
                let i32_type = self.context.i32_type();
                let (int_value, int_other_value) = (
                    i32_type.const_int(value as u64, false),
                    i32_type.const_int(other as u64, false),
                );
                Ok(self.builder.build_int_compare(operator, int_value, int_other_value, "comp"))
            }
            (TypeValues::Identifier(ident), TypeValues::I32(number)) => {
                let get_ident = self.get_ident(&ident)?;
                todo!()
            }
            _ => Err("the statement given couldn't be compared, when comparing values they must be of the same type.".into())
        }
    }
}
