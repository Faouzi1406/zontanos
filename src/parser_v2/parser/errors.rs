use super::Parser;

impl Parser {
    pub fn expected_type(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a type after ':' on line {}",
            current.line
        );
        msg
    }

    pub fn expected_array_value_comma(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected an array value, but got a comma, consider putting a comma in between array values, on line {}",
            current.line
        );
        msg
    }

    pub fn expected_array_generic(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected an array generic type: array<T>, but got array without generic type  on line {}",
            current.line
        );
        msg
    }

    pub fn expected_end_expr(&mut self, to: &str, end: &str) -> String {
        self.pos -= 1;
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a end to {to} '{end}' on line {}",
            current.line
        );
        msg
    }

    pub fn comma_without_type_generic(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a comma after a type <T, T> on line {}",
            current.line
        );
        msg
    }

    pub fn invalid_token_in_expr(&mut self, expr: &str, expected: &str) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Found a invalid token while parsing {expr} on line {}, expected a {expected} got {}",
            current.line, current.value
        );
        msg
    }

    pub fn expected_assign_token(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected assignment token on line {}",
            current.line
        );
        msg
    }

    pub fn expected_ident(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a variable identifier on line {}",
            current.line
        );
        msg
    }

    pub fn expected_type_seperator(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a type seperator ':' on line {} for {}",
            current.line, current.value
        );
        msg
    }

    pub fn expected_value_seprator(&mut self) -> String {
        let current = self.assert_prev_token();
        let msg = format!(
            "[Parse Error] Expected a value seperator ',' on line {}",
            current.line
        );
        msg
    }

    pub fn invalid_expected_type(&mut self, type_expected: &str, value_received: &str) -> String {
        let msg =
            format!("[Parse Error] Expected a type of {type_expected} but got {value_received}",);
        msg
    }
}
