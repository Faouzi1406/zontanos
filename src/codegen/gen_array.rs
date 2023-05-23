use super::CodeGen;
use crate::ast::r#return;
use crate::ast::types::MarkerTypes;
use crate::ast::types::VarTypes;
use crate::ast::variable::VarData;
use crate::ast::variable::Variable;
use inkwell::types::FloatType;
use inkwell::types::IntType;
use inkwell::values::ArrayValue;
use inkwell::values::IntValue;

impl CodeGen<'_> {
    pub(super) fn gen_array(&self, variable: &Variable) -> Result<(), String> {
        let var_name = &variable.var_name;

        let VarTypes::Array { array, array_type } = variable.var_type.clone() else {
            panic!("Gen array should never be called if the type of the variable isn't of and Array.")
        };

        let array_length = self.context.i8_type();
        let len = array_length.const_int(array.len() as u64, false);

        match &array_type {
            MarkerTypes::I8 => {
                let i8  = self.context.i8_type();
                let name = var_name.to_string();
                let array = Self::gen_i8_array_value(i8, array)?;
                let array_type = array.get_type();
                self.create_variable(array_type, var_name.to_string(), array);
            }
            MarkerTypes::I32 => {
                let i32  = self.context.i32_type();
                let name = var_name.to_string();
                let array = Self::gen_i32_array_value(i32, array)?;
                let array_type = array.get_type();
                self.create_variable(array_type, var_name.to_string(), array);
            }
            MarkerTypes::F32 => {
                let f32 = self.context.f32_type();
                let name = var_name.to_string();
                let array = Self::gen_float_array_value(f32, array)?;
                let array_type = array.get_type();
                self.create_variable(array_type, var_name.to_string(), array);
            }
            MarkerTypes::Char => {
                let char = self.context.i8_type();
                let name = var_name.to_string();
                let array = Self::gen_i8_array_value(char,array)?;
                let array_type = array.get_type();
                self.create_variable(array_type, var_name.to_string(), array);
            }
            MarkerTypes::String => {}
            unsupported => 
                unreachable!("Found array type {}, perhaps it is now supported in parsing but it is not in compiling.", array_type.to_string())
        }

        Ok(())
    }

    pub(super) fn gen_i32_array_value(typeof_int: IntType, arr: Vec<VarTypes>) -> Result<ArrayValue, String> {
        let mut values = Vec::new();

        for value in arr {
            let VarTypes::I32(value) = value else {
                return Err(Self::expected_array_value_but_got("i32"));
            };
            let value = typeof_int.const_int(value as u64, false);
            values.push(value)
        }

        let const_array = typeof_int.const_array(&values);
        Ok(const_array)
    }

    pub(super) fn gen_i8_array_value(typeof_int: IntType, arr: Vec<VarTypes>) -> Result<ArrayValue, String> {
        let mut values = Vec::new();

        for value in arr {
            let value = match value {
                VarTypes::I8(value) => typeof_int.const_int(value as u64, false),
                VarTypes::Char(value) => typeof_int.const_int(value.into(), false),
                _ => return Err(Self::expected_array_value_but_got("i8")),
            };
            values.push(value);
        }

        let const_array = typeof_int.const_array(&values);
        Ok(const_array)
    }

    pub(super)fn gen_float_array_value(typeof_float: FloatType, arr: Vec<VarTypes>) -> Result<ArrayValue, String> {
        let mut values = Vec::new();

        for value in arr {
            let VarTypes::F32(value) = value else {
                return Err(Self::expected_array_value_but_got("f32"));
            };
            let value = typeof_float.const_float(value as f64);
            values.push(value)
        }

        let const_array = typeof_float.const_array(&values);
        Ok(const_array)
    }

    fn not_and_array() -> String {
        let msg = format!("A call to gen_array was made but the value type wasn't and array");
        msg
    }

    fn expected_array_value_but_got(expected: &str) -> String {
        let msg = format!("Expected a value of type {expected} but this was not the case");
        msg
    }
}
