use std::{format, str::FromStr};

use crate::zon_parser::lexer::Keywords;

use super::{function_call::FunctionCall, variable::VarErrors};

#[derive(PartialEq, Debug, Clone)]
pub enum VarTypes {
    Array {
        array: Vec<VarTypes>,
        array_type: MarkerTypes,
    },
    FunctionCall(FunctionCall, MarkerTypes),
    I32(i32),
    F32(f32),
    U8(u8),
    I8(i8),
    Char(char),
    String(String),
    Bool(bool),
    /// Contains the name and the expected type of a identifier
    Identifier(String, MarkerTypes),
    None,
}

/// These are the marker types of the language.
///
/// They don't contain any value, just the type expected.
///
/// It implements FromStr, where the string is the type of the marker.
#[derive(Debug, PartialEq, Clone)]
pub enum MarkerTypes {
    Array(Box<MarkerTypes>),
    /// No marker type was found
    None,
    /// "string"
    String,
    /// "void"
    Void,
    /// "identifier"
    Identifier,
    /// "char"
    Char,
    /// "i32"
    I32,
    /// "f32"
    F32,
    /// "u8"
    U8,
    /// "i8"
    I8,
    /// "string"
    PointerString,
    /// "identifier"
    PointerIdentifier,
    /// "char"
    PointerChar,
    /// "i32"
    PointerI32,
    /// "f32"
    PointerF32,
    /// "u8"
    PointerU8,
    /// "i8"
    PointerI8,
}

impl ToString for MarkerTypes {
    fn to_string(&self) -> String {
        match self {
            Self::Char => "char".into(),
            Self::None => "None".into(),
            MarkerTypes::String => "string".into(),
            MarkerTypes::Void => "void".into(),
            MarkerTypes::Identifier => "identifier".into(),
            MarkerTypes::I32 => "i32".into(),
            MarkerTypes::F32 => "f32".into(),
            MarkerTypes::U8 => "u8".into(),
            MarkerTypes::I8 => "i8".into(),
            MarkerTypes::Array(arr) => format!("array:{}", arr.to_string()),
            MarkerTypes::PointerU8 => "^u8".into(),
            MarkerTypes::PointerI8 => ",^i8".into(),
            MarkerTypes::PointerI32 => "^i32".into(),
            MarkerTypes::PointerF32 => "^f32".into(),
            MarkerTypes::PointerChar => "^char".into(),
            MarkerTypes::PointerString => "^string".into(),
            MarkerTypes::PointerIdentifier => "^identifier".into(),
        }
    }
}

impl From<Keywords> for MarkerTypes {
    fn from(value: Keywords) -> Self {
        match value {
            Keywords::I8 => Self::I8,
            Keywords::U8 => Self::U8,
            Keywords::Char => Self::Char,
            Keywords::I32 => Self::I32,
            Keywords::F32 => Self::F32,
            Keywords::Void => Self::Void,
            Keywords::String => Self::String,
            _ => MarkerTypes::None,
        }
    }
}

impl FromStr for MarkerTypes {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "string" => Ok(MarkerTypes::String),
            "char" => Ok(MarkerTypes::Char),
            "identifier" => Ok(MarkerTypes::Identifier),
            "i32" => Ok(MarkerTypes::I32),
            "f32" => Ok(MarkerTypes::F32),
            "u8" => Ok(MarkerTypes::U8),
            "i8" => Ok(MarkerTypes::I8),
            "^string" => Ok(MarkerTypes::PointerString),
            "^char" => Ok(MarkerTypes::PointerChar),
            "^identifier" => Ok(MarkerTypes::PointerIdentifier),
            "^i32" => Ok(MarkerTypes::PointerI32),
            "^f32" => Ok(MarkerTypes::PointerF32),
            "^u8" => Ok(MarkerTypes::PointerU8),
            "^i8" => Ok(MarkerTypes::PointerI8),
            value => Err(format!("Type {value} is not a type")),
        }
    }
}

impl VarTypes {
    pub fn is_some(&self) -> bool {
        self != &VarTypes::None
    }

    pub fn is_none(&self) -> bool {
        self == &VarTypes::None
    }

    pub fn from_str(value: &str, type_expected: &str, line: usize) -> Result<VarTypes, VarErrors> {
        match type_expected {
            "i32" => {
                let parse: Result<i32, _> = value.parse();
                match parse {
                    Ok(value) => {
                        Ok(VarTypes::I32(value))
                    }
                    Err(_) => {
                        Err(VarErrors::IncorrectType(
                            format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                            line,
                        ))
                    }
                }
            }
            "f32" => {
                let parse: Result<f32, _> = value.parse();
                match parse {
                    Ok(value) => {
                        Ok(VarTypes::F32(value))
                    }
                    Err(_) => {
                        Err(VarErrors::IncorrectType(
                            format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                            line,
                        ))
                    }
                }
            }
            "u8" => {
                let parse: Result<u8, _> = value.parse();
                match parse {
                    Ok(value) => {
                        Ok(VarTypes::U8(value))
                    }
                    Err(_) => {
                        Err(VarErrors::IncorrectType(
                            format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                            line,
                        ))
                    }
                }
            }
            "i8" => {
                let parse: Result<i8, _> = value.parse();
                match parse {
                    Ok(value) => {
                        Ok(VarTypes::I8(value))
                    }
                    Err(_) => {
                        Err(VarErrors::IncorrectType(
                            format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                            line,
                        ))
                    }
                }
            }
            "char" => {
                let parse: Result<char, _> = value.parse();
                match parse {
                    Ok(value) => {
                        Ok(VarTypes::Char(value))
                    }
                    Err(_) => {
                        Err(VarErrors::IncorrectType(
                            format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                            line,
                        ))
                    }
                }
            }
            "string" => {
                let parse: Result<String, _> = value.parse();
                match parse {
                    Ok(value) => {
                        Ok(VarTypes::String(value))
                    }
                    Err(_) => {
                        Err(VarErrors::IncorrectType(
                            format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                            line,
                        ))
                    }
                }
            }
            "identifier" => Ok(VarTypes::Identifier(value.to_string(), MarkerTypes::None)),
            _ => Err(VarErrors::NotAType(type_expected.into(), line)),
        }
    }
}
