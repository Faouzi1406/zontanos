use std::println;

use inkwell::{
    module::Linkage,
    types::BasicMetadataTypeEnum,
    values::{BasicMetadataValueEnum, FunctionValue},
    AddressSpace,
};

use crate::{ast::types::MarkerTypes, codegen::CodeGen};

pub(crate) trait StdFunctions {
    fn gen_std_func(
        &self,
        args: Vec<BasicMetadataValueEnum>,
        scope: FunctionValue,
        var_name: Option<&str>,
        name: &str,
    ) -> Result<MarkerTypes, String>;
    fn gen_std_printf(&self, scope: FunctionValue) -> Result<(), String>;
}

impl StdFunctions for CodeGen<'_> {
    /// Returns the return type of the std function it parsed or a err string, return type can be
    /// used to check if it is equal to that of the variable if it is a variable;
    fn gen_std_func(
        &self,
        args: Vec<BasicMetadataValueEnum>,
        scope: FunctionValue,
        var_name: Option<&str>,
        call_name: &str,
    ) -> Result<MarkerTypes, String> {
        match call_name {
            "printf" => {
                if let Some(print_f) = self.module.get_function("printf") {
                    let Some(first_arg) = args.first() else {
                        return  Err("expected first printf argument to be a string".to_string());
                    };
                    let BasicMetadataValueEnum::PointerValue(_) = first_arg else  {
                        return  Err("expected first printf argument to be a string".to_string());
                    };

                    if let Some(var_name) = var_name {
                        self.builder.build_call(print_f, &args, var_name);
                    } else {
                        self.builder.build_call(print_f, &args, "");
                    }

                    return Ok(MarkerTypes::I32);
                } else {
                    self.gen_std_printf(scope);
                    return self.gen_std_func(args, scope, var_name, call_name);
                };
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