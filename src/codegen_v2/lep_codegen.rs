use std::collections::LinkedList;

use inkwell::{
    values::{BasicValueEnum, IntValue},
    IntPredicate,
};

use super::{CodeGen, CompileResult};
use crate::parser_v2::{
    ast::{TypeValues, Value, NodeTypes},
    parser::lep::{LogicalStatement, Statements},
};

impl<'ctx> CodeGen<'ctx> {
    pub fn gen_logcal_statement(&self, logical_statement: &'ctx Box<LogicalStatement>, in_block: Option<&str>) -> CompileResult<()> {
        let case = self.gen_case(&logical_statement.case, in_block)?;
        let (func, _) = self.scope.unwrap();
        let Some(entry) = func.get_first_basic_block() else {
            return Err("expected to be in entry".into());
        };

        let NodeTypes::Block(if_block) = &logical_statement.if_do else {
            return Err("expected a if block".into());
        };
        let Some(NodeTypes::Block(else_block)) = &logical_statement.else_do else {
            return Err("as of right now a if block must have a else block, this will get changed. :(".into())
        };

        let if_do = self.gen_block(func, &if_block , Some("if_then_do"))?;
        self.builder.position_at_end(entry);
        let else_do = self.gen_block(func, &else_block, Some("else_do"))?;
        self.builder.position_at_end(entry);
        self.builder.build_conditional_branch(case, if_do, else_do);
        Ok(())
    }

    fn gen_case(&self, statements: &'ctx Vec<Statements>, in_block: Option<&str>) -> CompileResult<IntValue> {
        let mut stack: LinkedList<IntValue> = LinkedList::new();
        let mut statements = statements.clone().iter();

        while let Some(statement) = statements.next() {
            if let Statements::Or = statement {
                let Some(lhs) = stack.pop_front() else {
                    return Err("can't generate a or case if there are no values to compare to".into());
                };
                let Some(rhs) = statements.next() else {
                    return Err("can't generate a or case if there are no values to compare to".into());
                };
                let rhs = self.statement_case(rhs, in_block)?;
                let comp = self.builder.build_or(lhs, rhs, "and");
                stack.push_front(comp);
                continue;
            }

            if let Statements::And = statement {
                let Some(lhs) = stack.pop_front() else {
                        return Err("can't generate a or case if there are no values to compare to".into());
                };
                let Some(rhs) = statements.next() else {
                        return Err("can't generate a or case if there are no values to compare to".into());
                };
                let rhs = self.statement_case(rhs, in_block)?;
                let comp = self.builder.build_and(lhs, rhs, "and");
                stack.push_front(comp);
                continue;
            }

            let value = self.statement_case(statement, in_block)?;
            stack.push_front(value);
        }
        let Some(statement) = stack.pop_front() else {
            return Err("got no cases".into());
        };

        Ok(statement)
    }

    fn statement_case(&self, statement: &'ctx Statements, in_block: Option<&str>) -> CompileResult<IntValue> {
        match statement {
            Statements::More(lhs, rhs) => {
                let value = self.gen_i32_case(lhs, rhs, IntPredicate::SGT, in_block)?;
                Ok(value)
            }
            Statements::Less(lhs, rhs) => {
                let value = self.gen_i32_case(lhs, rhs, IntPredicate::SLT, in_block)?;
                Ok(value)
            }
            Statements::LessEq(lhs, rhs) => {
                let value = self.gen_i32_case(lhs, rhs, IntPredicate::SLE, in_block)?;
                Ok(value)
            }
            Statements::MoreEq(lhs, rhs) => {
                let value = self.gen_i32_case(lhs, rhs, IntPredicate::SGE, in_block)?;
                Ok(value)
            }
            Statements::OrOr(lhs, rhs) => {
                let value = self.gen_i32_case(lhs, rhs, IntPredicate::EQ, in_block)?;
                Ok(value)
            }
            _ => todo!(),
        }
    }

    fn gen_i32_case(
        &self,
        value: &Value,
        or: &Value,
        operator: IntPredicate,
        in_block: Option<&str>
    ) -> CompileResult<IntValue> {
        match (&value.value, &or.value) {
            (TypeValues::I32(value), TypeValues::I32(other)) => {
                let i32_type = self.context.i32_type();
                let (int_value, int_other_value) = (
                    i32_type.const_int(*value as u64, false),
                    i32_type.const_int(*other as u64, false),
                );
                Ok(self.builder.build_int_compare(operator, int_value, int_other_value, "comp"))
            }
            (TypeValues::Identifier(ident), TypeValues::I32(number)) => {
                let get_ident = self.get_ident(&ident, in_block)?;
                if get_ident.is_pointer_value() {
                    let load_ident = self.builder.build_load(get_ident.into_pointer_value(), "if_load");
                    let BasicValueEnum::IntValue(value) = load_ident else {
                        return Err("when comparing expected value to be integer".into());
                    };
                    let i32_type = self.context.i32_type();
                    let i32_value = i32_type.const_int(*number as u64, false);
                    Ok(self.builder.build_int_compare(operator, value, i32_value, "comp"))
                } else {
                    if get_ident.is_int_value() {
                        let int_value = get_ident.into_int_value();
                        let i32_type = self.context.i32_type();
                        let i32_value = i32_type.const_int(*number as u64, false);
                        Ok(self.builder.build_int_compare(operator, int_value, i32_value,  "comp"))
                    } else {
                        Err("Can't compare none integer values".into())
                    }
                }
            }
            (TypeValues::I32(number), TypeValues::Identifier(ident)) => {
                let get_ident = self.get_ident(&ident, in_block)?;
                if get_ident.is_pointer_value() {
                    let load_ident = self.builder.build_load(get_ident.into_pointer_value(), "if_load");
                    let BasicValueEnum::IntValue(value) = load_ident else {
                        return Err("when comparing expected value to be integer".into());
                    };
                    let i32_type = self.context.i32_type();
                    let i32_value = i32_type.const_int(*number as u64, false);
                    Ok(self.builder.build_int_compare(operator, i32_value, value, "comp"))
                } else {
                    if get_ident.is_int_value() {
                        let int_value = get_ident.into_int_value();
                    let i32_type = self.context.i32_type();
                    let i32_value = i32_type.const_int(*number as u64, false);
                        Ok(self.builder.build_int_compare(operator, i32_value, int_value, "comp"))
                    } else {
                        Err("Can't compare none integer values".into())
                    }
                }
            }
            (TypeValues::Identifier(lhv), TypeValues::Identifier(rhv)) => {
                let (lhv, rhv) = (self.get_ident(&lhv, in_block)?, self.get_ident(&rhv, in_block)?);
                if lhv.is_pointer_value() && rhv.is_pointer_value() {
                    let (load1, load2) = 
                        (self.builder.build_load(lhv.into_pointer_value(), "load1"), self.builder.build_load(rhv.into_pointer_value(), "load2"));
                    let (lhv, rhv) = (load1.into_int_value(), load2.into_int_value());
                    Ok(self.builder.build_int_compare(operator,  rhv, lhv,  "comp"))
                } else if lhv.is_int_value() && rhv.is_int_value() {
                    Ok(self.builder.build_int_compare(operator,   rhv.into_int_value(), lhv.into_int_value(),  "comp"))
                } else {
                    Err("Can't compare none integer values".into())
                }
            }
            _ => Err("the statement given couldn't be compared, when comparing values they must be of the same type.".into())
        }
    }
}
