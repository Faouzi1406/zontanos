use crate::zon_parser::lexer::Tokens;

pub enum ParseErrors {
    /// If a value never got assigned  
    /// .0 = line
    NoAssignment(usize),
    /// If a previous token was expected but no token was received
    NoPrevToken,
    /// If a previous token was expected to be of type .0 but got wrong type .1
    /// .0 = expected
    /// .1 = received
    WrongToken(Tokens, Tokens),
    /// If a next token was expected but the next token was None
    /// .0 = line
    ExpectedNext(usize),
    /// If a type/value was expected, but token received was not a type/value
    /// .0 = line
    /// .1 = Type expected
    ExpectedType(usize),
}

impl ToString for ParseErrors {
    fn to_string(&self) -> String {
        match self {
            ParseErrors::NoAssignment(line) => {
                format!("Expected a identifier, but didn't get one, on line {line}")
            }
            ParseErrors::NoPrevToken => {
                format!("Expected a token, but didn't get one")
            }
            ParseErrors::WrongToken(expected, received) => {
                format!(
                    "Expected token of type {:#?}, but got token of type {:#?}",
                    expected, received
                )
            }
            ParseErrors::ExpectedNext(line) => {
                format!("Expected the next token to be some, but got none, on line {line}",)
            }
            // Todo: Add error handling based on it's type to improve errors
            ParseErrors::ExpectedType(line) => {
                format!(
                    "Expected the next token to be a value, but it wasn't a value, on line {line}"
                )
            }
        }
    }
}
