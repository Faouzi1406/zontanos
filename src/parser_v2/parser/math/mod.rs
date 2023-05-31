use crate::parser_v2::parser::Operator;
use std::mem::swap;

use crate::zon_parser::lexer::Tokens;

use super::Parser;

#[derive(Clone, Debug)]
pub struct Stack {
    value: Option<Tokens>,
    head: Option<Box<Stack>>,
    next: Option<Box<Stack>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Que {
    pub value: Option<Tokens>,
    pub next: Option<Box<Que>>,
}

impl Que {
    pub fn init() -> Self {
        Self {
            value: None,
            next: None,
        }
    }

    pub fn new(value: Tokens) -> Self {
        Self {
            value: Some(value),
            next: None,
        }
    }

    pub fn append(&mut self, value: Tokens) {
        if self.value.is_none() {
            self.value = Some(value);
            return;
        }

        if self.next.is_some() {
            return self.next.as_mut().unwrap().append(value);
        }

        let value = Box::new(Que::new(value));
        self.next = Some(value);
    }

    pub fn pop(&mut self) -> Tokens { 
        let prev = &self;
        let value = self;
        while let Some(value) = &mut value.next {
        };
        todo!()
    }
}

impl Stack {
    pub fn init() -> Self {
        Self {
            value: None,
            head: None,
            next: None,
        }
    }
    pub fn new(value: Tokens) -> Self {
        Self {
            value: Some(value),
            head: None,
            next: None,
        }
    }

    pub fn push(&mut self, value: Tokens) {
        if self.value.is_none() {
            self.value = Some(value);
            return;
        }
        if self.next.is_some() {
            return self.next.as_mut().unwrap().push(value);
        }
        self.next = Some(Box::new(Self::new(value)));
    }

    pub fn pop(&mut self) -> Option<Tokens> {
        let value = self.value?;

        if self.next.is_some() {
            let next = self.next.as_mut().unwrap();
            self.value = next.value;

            let mut next = next.next.clone();
            if self.next.as_ref().unwrap().next.is_some() {
                swap(&mut self.next, &mut next);
                return Some(value);
            }

            self.next = None;
            return Some(value);
        } else {
            self.value = None;
            return Some(value);
        }
    }
}

impl Parser {
    // Expects all values to be numbers or operators :)
    pub fn shunting_yard_parse_math_expr(&mut self) {
        let stack = Stack::init();
        let que = Que::init();
        while let Some(token) = self.next() {
            match token.token_type {
                Tokens::Number => {}
                _ => return,
            }
        }
    }
}
