use crate::ast::{r#return::Return, types::MarkerTypes};

use super::parser::Parser;

impl Parser {
    pub fn parse_ret(&mut self, marker_type: MarkerTypes) -> Result<Return, String> {
        let Some(return_value) = self.next() else {
            return Err(format!("Expected a return value but there wasn't one"));
        };
        let get_return_value = self.parse_value(return_value, marker_type)?;
        Ok(Return(get_return_value))
    }
}
