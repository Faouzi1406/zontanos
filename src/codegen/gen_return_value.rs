use inkwell::values::BasicValue;

use crate::{
    ast::{
        r#return::Return,
        types::{MarkerTypes, VarTypes},
    },
    codegen::CodeGen,
};

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn get_int_return_value(&self, return_value: &Return) -> Option<impl BasicValue> {
        match return_value.0 {
            VarTypes::U8(value) => {
                let i8 = self.context.i8_type();
                let value = i8.const_int(value as u64, false);
                return Some(value);
            }
            VarTypes::I8(value) => {
                let i8 = self.context.i8_type();
                let value = i8.const_int(value as u64, false);
                return Some(value);
            }
            VarTypes::Char(value) => {
                let i8 = self.context.i8_type();
                let value = i8.const_int(value as u64, false);
                return Some(value);
            }
            VarTypes::I32(value) => {
                let i32 = self.context.i32_type();
                let value = i32.const_int(value as u64, false);
                return Some(value);
            }
            _ => None,
        }
    }

    pub(super) fn gen_float_return_type(&self, return_value: &Return) -> Option<impl BasicValue> {
        match return_value.0 {
            VarTypes::F32(float_value) => {
                let f32 = self.context.f32_type();
                let value = f32.const_float(float_value as f64);
                return Some(value);
            }
            _ => None,
        }
    }

    pub(super) fn gen_arr_return_type(&self, return_value: &'ctx Return) -> Option<impl BasicValue> {
        match &return_value.0 {
            VarTypes::Array { array, array_type } => match array_type {
                MarkerTypes::I8 => {
                    let i8 = self.context.i8_type();
                    let Ok(array_value) = CodeGen::gen_i8_array_value(i8, array) else {
                        return None;
                    };
                    Some(array_value)
                }
                MarkerTypes::I32 => {
                    let i32 = self.context.i32_type();
                    let Ok(array_value) = CodeGen::gen_i32_array_value(i32, array) else {
                        return None;
                    };
                    Some(array_value)
                }
                MarkerTypes::F32 => {
                    let f32 = self.context.f32_type();
                    let Ok(array_value) = CodeGen::gen_float_array_value(f32, array) else {
                        return None;
                    };
                    Some(array_value)
                }
                MarkerTypes::Char => {
                    let i32 = self.context.i8_type();
                    let Ok(array_value) = CodeGen::gen_i8_array_value(i32, array) else {
                        return None;
                    };
                    Some(array_value)
                }
                _ => None,
            },
            VarTypes::String(value) => {
                let string = self.context.const_string(value.as_bytes(), false);
                return Some(string);
            }
            _ => None,
        }
    }
}
