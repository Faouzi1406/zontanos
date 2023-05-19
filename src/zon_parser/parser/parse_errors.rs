use crate::zon_parser::lexer::Tokens;

pub enum ParseErrors {
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
    /// If a token wasn't expected to be for example in a block InvalidToken is returned;
    /// .0 = line
    /// .1 = Token that was invalid
    InvalidToken(usize, Tokens),
    /// If there was no end to a certain AstNodeType,
    ///
    /// # Example
    /// {
    /// // we open the block, but never close it
    /// let helloworld = "helloworld"
    /// ...
    ///
    /// .0 = line
    /// .1 = The node type that didn't get a close
    NoEnd(usize),
}

// Todo: Most of these errors are just place holder now and will get changed to be more helpful
impl ToString for ParseErrors {
    fn to_string(&self) -> String {
        match self {
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
            ParseErrors::InvalidToken(line, invalid_token) => {
                format!(
                    "got a invalid token on line {line}, {invalid_token:#?} is not allowed in this context."
                )
            }
            ParseErrors::NoEnd(line) => {
                format!("Expected a end to  on line {line}, but never got one")
            }
        }
    }
}
