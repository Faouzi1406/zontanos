//! src/ast
//!
//! This will contain the Ast of the language.
//! The Ast will be used to compile the language into llvm-ir;
#![allow(dead_code)]
pub mod variable {
    use std::fmt::Display;

    #[derive(PartialEq)]
    pub enum VarTypes {
        I32(i32),
        F32(f32),
        U8(u8),
        I8(i8),
        String(String),
        None,
    }

    #[derive(Debug)]
    pub enum VarErrors {
        /// Error would take place if the type is different from the value;
        /// # Example
        ///
        /// U8(400); // This could never happen since a u8 can never be bigger than 255 therefore a [`ParseError`]
        ///
        /// .0 = variable name
        /// .1 = cause
        /// .2 = line
        ParseError(String, String, u32),
        /// This error would take place if a name gets set of a variable that already has a name;
        /// # Example
        ///
        /// let other other = 10; // User might have accidentally typed other 2x leading to the
        /// parser thinking that it's just two seperate assignments, this is of course not the
        /// case;
        ///
        /// .0 = variable name
        /// .1 = line
        NameNotEmpty(String, u32),
        /// error happens if a variable already has a type;
        ///
        /// # example
        ///
        ///  let some_value = 1 2; // this would lead to the variable being assigned to different
        ///  values in one statement, this is of course not possible.
        ///
        /// .0 = variable name
        /// .1 = line
        VarHasType(String, u32),
        /// Error happens if a type doesn't exist;
        ///
        /// # example
        ///
        ///  let some_value: no_type = "whut";  // no_type is not a real type therefore
        ///  [`NotAType`] is returned;
        ///
        /// .0 = type
        /// .1 = line
        NotAType(String, u32),
        /// Error happens if the incorrect type is being assigned;
        ///
        /// # example
        ///
        ///  let some_value: u8 = 10000000; // a u8 can never have a value of 10000000;
        ///  therefore [`IncorrectType`] is returned;
        ///
        /// .0 = type
        /// .1 = line
        IncorrectType(String, u32),
    }

    impl VarTypes {
        fn is_some(&self) -> bool {
            self != &VarTypes::None
        }

        fn is_none(&self) -> bool {
            self == &VarTypes::None
        }

        fn from_str(value: &str, type_expected: &str, line: u32) -> Result<VarTypes, VarErrors> {
            match type_expected {
                "i32" => {
                    let parse: Result<i32, _> = value.parse();
                    match parse {
                        Ok(value) => {
                            return Ok(VarTypes::I32(value));
                        }
                        Err(_) => {
                            return Err(VarErrors::IncorrectType(
                                format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                                line,
                            ));
                        }
                    }
                }
                "f32" => {
                    let parse: Result<f32, _> = value.parse();
                    match parse {
                        Ok(value) => {
                            return Ok(VarTypes::F32(value));
                        }
                        Err(_) => {
                            return Err(VarErrors::IncorrectType(
                                format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                                line,
                            ));
                        }
                    }
                }
                "u8" => {
                    let parse: Result<u8, _> = value.parse();
                    match parse {
                        Ok(value) => {
                            return Ok(VarTypes::U8(value));
                        }
                        Err(_) => {
                            return Err(VarErrors::IncorrectType(
                                format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                                line,
                            ));
                        }
                    }
                }
                "string" => {
                    let parse: Result<String, _> = value.parse();
                    match parse {
                        Ok(value) => {
                            return Ok(VarTypes::String(value));
                        }
                        Err(_) => {
                            return Err(VarErrors::IncorrectType(
                                format!("type that was expected was {type_expected}, value wasn't a valid {type_expected}"),
                                line,
                            ));
                        }
                    }
                }
                _ => Err(VarErrors::NotAType(type_expected.into(), line)),
            }
        }
    }

    impl Display for VarErrors {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let reason = match &self {
                VarErrors::ParseError(name, reason, line) => {
                    format!("Error variable {name}; {reason} on line {line}")
                }
                VarErrors::NameNotEmpty(name, line) => format!("variable {name} was already assigned; consider removing the second assignment on line {line}"),
                VarErrors::VarHasType(name, line) => format!("variable {name} already already has a type; consider removing the second type assign on line {line}"),
                VarErrors::IncorrectType(reason, line) => format!("Error: {reason} on line {line}"),
                VarErrors::NotAType(var_type, line) => format!("Error: got {var_type} on line {var_type}; but {var_type} is not a type. on line {line}"),
            };
            write!(f, "Error: {:#?}; reason {reason}", &self)
        }
    }

    /// Represents the structure of a Variable
    ///
    /// Use the method build on this struct to get the data within it.
    /// This is because the methods make sure the data you are receiving is valid data.
    pub struct Variable {
        var_name: String,
        var_type: VarTypes,
        var_line: u32,
        is_constant: bool,
    }


    impl Default for Variable {
        fn default() -> Self {
            Self {
                var_name: String::new(),
                var_type: VarTypes::None,
                var_line: 0,
                is_constant: false
            }
        }
    }

    pub trait VarData {
        /// Sets the name of the variable struct, returns a error if the variable already has a
        /// name. Takes a optional line, if no line is provided it takes that of the variable.
        fn set_name(&mut self, name: String, line: Option<u32>) -> Result<(), VarErrors>;
        /// Gets the name of a variable, returns None if the variable name is of length 0
        fn get_name(&self) -> Option<&str>;
        // Sets the type of a variable, this type gets checked,  If the variable already has type this would also cause a
        // error to be returned;
        fn set_type(&mut self, value: VarTypes, line: Option<u32>) -> Result<(), VarErrors>;
        // Change the type of variable to a constant variable 
        fn set_to_constant(&mut self); 
        // Change the type of variable to a normal (not constant) variable 
        fn set_to_not_constant(&mut self); 
    }

    impl VarData for Variable {
        fn set_name(&mut self, name: String, line: Option<u32>) -> Result<(), VarErrors> {
            if self.var_name.is_empty() {
                self.var_name = name;
                return Ok(());
            }
            Err(VarErrors::NameNotEmpty(
                self.var_name.clone(),
                line.unwrap_or(self.var_line),
            ))
        }

        fn get_name(&self) -> Option<&str> {
            if self.var_name.is_empty() {
                return None;
            }
            Some(&self.var_name)
        }

        fn set_type(&mut self, value: VarTypes, line: Option<u32>) -> Result<(), VarErrors> {
            if self.var_type.is_none() {
                self.var_type = value;
                return Ok(());
            }
            Err(VarErrors::VarHasType(
                self.var_name.clone(),
                line.unwrap_or(self.var_line),
            ))
        }
        fn set_to_constant(&mut self) {
           self.is_constant = true; 
        }
        fn set_to_not_constant(&mut self) {
           self.is_constant = false; 
        }
    }
}
