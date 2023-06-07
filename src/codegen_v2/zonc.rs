use inkwell::{module::Linkage, values::FunctionValue};

use super::CodeGen;

pub(super) trait GenC<'ctx> {
    fn gen_printf(&self) -> FunctionValue<'ctx>;
    fn gen_abs(&self) -> FunctionValue<'ctx>;
    fn gen_getchar(&self) -> FunctionValue<'ctx>;
    fn gen_putchar(&self) -> FunctionValue<'ctx>;
    fn gen_c_function(&self, name: &str) -> Option<FunctionValue<'ctx>>;
}

impl<'ctx> GenC<'ctx> for CodeGen<'ctx> {
    fn gen_printf(&self) -> FunctionValue<'ctx> {
        let printf_args = [self.context.i8_type().ptr_type(Default::default()).into()];
        let printf_type = self.context.i32_type().fn_type(&printf_args, true);
        let printf = self
            .module
            .add_function("printf", printf_type, Some(Linkage::External));
        printf
    }

    fn gen_abs(&self) -> FunctionValue<'ctx> {
        let abs_args = [self.context.i32_type().into()];
        let abs_type = self.context.i32_type().fn_type(&abs_args, false);
        let readf = self
            .module
            .add_function("abs", abs_type, Some(Linkage::External));
        readf
    }

    fn gen_getchar(&self) -> FunctionValue<'ctx> {
        let getchar_type = self.context.i8_type().fn_type(&[], false);
        let getchar = self
            .module
            .add_function("getchar", getchar_type, Some(Linkage::External));
        getchar
    }

    fn gen_putchar(&self) -> FunctionValue<'ctx> {
        let putchar_type = self
            .context
            .i8_type()
            .fn_type(&[self.context.i8_type().into()], false);
        let putchar = self
            .module
            .add_function("putchar", putchar_type, Some(Linkage::External));
        putchar
    }

    fn gen_c_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        match name {
            "printf" => Some(self.gen_printf()),
            "abs" => Some(self.gen_abs()),
            "getchar" => Some(self.gen_getchar()),
            "putchar" => Some(self.gen_putchar()),
            _ => None,
        }
    }
}
