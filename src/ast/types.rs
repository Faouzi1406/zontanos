use super::variable::VarErrors;

#[derive(PartialEq)]
pub enum VarTypes {
    Array { array: Vec<VarTypes> },
    I32(i32),
    F32(f32),
    U8(u8),
    I8(i8),
    String(String),
    Identifier(String),
    None,
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
            _ => Err(VarErrors::NotAType(type_expected.into(), line)),
        }
    }
}
