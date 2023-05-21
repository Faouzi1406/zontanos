use std::todo;

use crate::ast::r#return::Return;

use super::parser::Parser;

impl Parser {
    pub fn parse_ret(&mut self) -> Result<Return, String> {
        let return_value = self.next();
        todo!()
    }
}
