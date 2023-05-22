use crate::ast::r#return::Return;

use super::parser::Parser;

impl Parser {
    pub fn parse_ret(&mut self) -> Result<Return, String> {
        let Some(return_value) = self.next() else {
            return Err(format!("Expected a return value but there wasn't one"));
        };
        let get_return_value = self.parse_value(return_value)?;
        Ok(Return(get_return_value))
    }
}
