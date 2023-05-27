use super::Types;

impl From<&str> for Types {
    fn from(value: &str) -> Self {
        match value {
            "i8" => Self::I8,
            "u8" => Self::U8,
            "i32" => Self::I32,
            "f32" => Self::F32,
            "char" => Self::Char,
            "string" => Self::String,
            "array" => Self::Array,
            unknown_type => Self::UnknownType(unknown_type.to_string())
        }
    }
}
