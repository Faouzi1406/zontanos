use std::println;

use inkwell::{
    module::Linkage,
    types::{BasicMetadataTypeEnum, IntType},
    values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum, CallSiteValue, FunctionValue},
    AddressSpace,
};

use crate::{ast::types::MarkerTypes, codegen::CodeGen};

pub(crate) trait StdFunctions<'ctx> {
    fn gen_std_func(
        &self,
        args: Vec<BasicMetadataValueEnum<'ctx>>,
        scope: FunctionValue,
        var_name: Option<&str>,
        name: &str,
    ) -> Result<(AnyValueEnum<'ctx>, MarkerTypes), String>;
    fn gen_std_printf(&self, scope: FunctionValue) -> Result<(), String>;
}

impl<'ctx> StdFunctions<'ctx> for CodeGen<'ctx> {
    /// Returns the return type of the std function it parsed or a err string, return type can be
    /// used to check if it is equal to that of the variable if it is a variable;
    fn gen_std_func(
        &self,
        args: Vec<BasicMetadataValueEnum<'ctx>>,
        scope: FunctionValue,
        var_name: Option<&str>,
        call_name: &str,
    ) -> Result<(AnyValueEnum<'ctx>, MarkerTypes), String> {
        let var_name = if let Some(var_name) = var_name {
            var_name
        } else {
            ""
        };
        match call_name {
            "printf" => {
                if let Some(print_f) = self.module.get_function("printf") {
                    let Some(first_arg) = args.first() else {
                        return  Err("expected first printf argument to be a string".to_string());
                    };
                    let BasicMetadataValueEnum::PointerValue(_) = first_arg else  {
                        return  Err("expected first printf argument to be a string".to_string());
                    };
                    let call = self.builder.build_call(print_f, &args, var_name);
                    return Ok((call.as_any_value_enum(), MarkerTypes::I32));
                } else {
                    self.gen_std_printf(scope);
                    return self.gen_std_func(args, scope, Some(var_name), call_name);
                };
            }
            "add" => {
                if args.len() > 2 {
                    return Err(format!("add expects two arguments but got {}", args.len()));
                }
                let (add_1, add_2) = (args.get(0).unwrap(), args.get(1).unwrap());
                match (add_1.as_any_value_enum(), add_2.as_any_value_enum()) {
                    (AnyValueEnum::IntValue(int0), AnyValueEnum::IntValue(int1)) => {
                        let add = self.builder.build_int_add(int0, int1, var_name);
                        let var = self.context.i32_type();
                        let alloc = self.builder.build_alloca(var, var_name);
                        self.builder.build_store(alloc, add);
                        return Ok((add.into(), MarkerTypes::I32));
                    }
                    _ => return Err("add expects two integers.".to_owned()),
                }
            }
            name => return Err(format!("No standard function called {name}")),
        }
    }

    fn gen_std_printf(&self, scope: FunctionValue) -> Result<(), String> {
        let print_f = &self.context.i32_type().fn_type(
            &[self
                .context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .into()],
            true,
        );

        let printf = &self
            .module
            .add_function("printf", *print_f, Some(Linkage::External));
        Ok(())
    }
}
