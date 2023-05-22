use inkwell::values::ArrayValue;
use inkwell::values::IntValue;
use crate::ast::r#return;
use crate::ast::types::MarkerTypes;
use crate::ast::types::VarTypes;
use crate::ast::variable::VarData;
use crate::ast::variable::Variable;
use super::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(super) fn gen_array(&'ctx self, variable: &'ctx Variable) -> Result<(), String> {
        let Some(var_name) = variable.get_name() else {
            return Err(Self::variable_no_name(variable.var_line));
        };

        let VarTypes::Array { array, array_type } = &variable.var_type else {
            panic!("Gen array should never be called if the type of the variable isn't of and Array.")
        };

        match array_type {
            MarkerTypes::I8 => {
                let i8  = self.context.i8_type();
                let name = var_name.to_string();
                let array = self.gen_i8_array(array)?;
                self.create_variable(i8, name, array);
            }
            MarkerTypes::I32 => {
                let i32  = self.context.i32_type();
                let name = var_name.to_string();
                let array = self.gen_i32_type(array)?;
                self.create_variable(i32, name, array);
            }
            MarkerTypes::F32 => {
                let f32 = self.context.i32_type();
                let name = var_name.to_string();
                let array = self.gen_f32_type(array)?;
                self.create_variable(f32, name, array);
            }
            MarkerTypes::Char => {
                let char = self.context.i8_type();
                let name = var_name.to_string();
                let array = self.gen_i8_array(array)?;
                self.create_variable(char, name, array);
            }
            MarkerTypes::String => {}
            unsupported => 
                unreachable!("Found array type {}, perhaps it is now supported in parsing but it is not in compiling.", array_type.to_string())
        }

        Ok(())
    }

    fn gen_i8_array(&self, arr: &'ctx Vec<VarTypes>) -> Result<ArrayValue, String> {
        let i8 = self.context.i8_type();
        let mut values = Vec::new();
        
        for value in arr {
            let VarTypes::I8(value) = value else {
                return Err(Self::expected_array_value_but_got("i8"));
            };
            let value = i8.const_int(*value as u64, false);
            values.push(value)
        }

        let const_array  = i8.const_array(&values);
        Ok(const_array)
    }

    fn gen_i32_type(&self, arr: &'ctx Vec<VarTypes>) -> Result<ArrayValue, String> {
        let i32 = self.context.i32_type();
        let mut values = Vec::new();
        
        for value in arr {
            let VarTypes::I32(value) = value else {
                return Err(Self::expected_array_value_but_got("i32"));
            };
            let value = i32.const_int(*value as u64, false);
            values.push(value)
        }

        let const_array  = i32.const_array(&values);
        Ok(const_array)
    }

    fn gen_f32_type(&self, arr: &'ctx Vec<VarTypes>) -> Result<ArrayValue, String> {
        let f32 = self.context.f32_type();
        let mut values = Vec::new();
        
        for value in arr {
            let VarTypes::F32(value) = value else {
                return Err(Self::expected_array_value_but_got("f32"));
            };
            let value = f32.const_float(*value as f64);
            values.push(value)
        }

        let const_array  = f32.const_array(&values);
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
