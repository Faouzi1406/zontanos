//!  Used to match Types::T against a value;

use super::{TypeValues, Types};

impl Types {
    // Returns if the value string can be parsed into it's type,
    // This returns false on arrays and identifiers
    pub fn types_match(&self, value: &str) -> bool {
        match &self {
            Self::I8 => value.parse::<i8>().is_ok(),
            Self::U8 => value.parse::<i8>().is_ok(),
            Self::String => value.parse::<String>().is_ok(),
            Self::Char => value.parse::<char>().is_ok(),
            _ => false,
        }
    }

    pub fn type_value_convert(&self, value: &str) -> Result<TypeValues, String> {
        match &self {
            Self::I8 => {
                let Ok(value) = value.parse::<i8>() else {
                    return  Err(format!("Expected type value i8 but got value {value}"));
                };
                Ok(TypeValues::I8(value))
            }
            Self::U8 => {
                let Ok(value) = value.parse::<u8>() else {
                    return  Err(format!("Expected type value u8 but got value {value}"));
                };
                Ok(TypeValues::U8(value))
            }
            Self::I32 => {
                let Ok(value) = value.parse::<i32>() else {
                    return  Err(format!("Expected type value u8 but got value {value}"));
                };
                Ok(TypeValues::I32(value))
            }
            Self::F32 => {
                let Ok(value) = value.parse::<f32>() else {
                    return  Err(format!("Expected type value u8 but got value {value}"));
                };
                Ok(TypeValues::F32(value))
            }
            Self::String => {
                let Ok(value) = value.parse::<String>() else {
                    return  Err(format!("Expected type value String but got value {value}"));
                };
                Ok(TypeValues::String(value))
            }
            Self::Char => {
                let Ok(value) = value.parse::<String>() else {
                    return  Err(format!("Expected type value String but got value {value}"));
                };
                Ok(TypeValues::String(value))
            }
            not_supported_conversion => panic!(
                "Type value conversion shouldn't get called on type {not_supported_conversion:#?}"
            ),
        }
    }
}
