use std::mem::swap;

use crate::zon_parser::lexer::{Operator, Tokens};

use super::Parser;

#[derive(Debug)]
pub struct Stack<T> {
    value: Option<T>,
    head: Option<Box<Stack<T>>>,
    next: Option<Box<Stack<T>>>,
}

impl<T> Clone for Stack<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            head: self.head.clone(),
            next: self.next.clone(),
        }
    }
}

impl<T> Stack<T>
where
    T: Clone,
{
    pub fn init() -> Self {
        Self {
            value: None,
            head: None,
            next: None,
        }
    }

    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
            head: None,
            next: None,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
            return;
        }
        if self.next.is_some() {
            return self.next.as_mut().unwrap().push(value);
        }
        self.next = Some(Box::new(Self::new(value)));
    }

    pub fn pop(&mut self) -> Option<T> {
        let value = self.value.clone()?;

        if self.next.is_some() {
            let next = self.next.as_mut().unwrap();
            self.value = next.value.clone();

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

    pub fn current(&mut self) -> &Option<T> {
        &self.value
    }

    pub fn peak(&mut self) -> &Option<T> {
        let Some(next) = self.next.as_ref() else {return &None};
        &next.value
    }
}

impl Parser {
    // Expects all values to be numbers or operators :)
    pub fn shunting_yard_parse_math_expr(&mut self) {
        todo!("Write this!");
        let mut num_stack: Stack<i32> = Stack::init();
        let op_stack: Stack<Operator> = Stack::init();

        while let Some(token) = self.next() {
            match token.token_type {
                Tokens::Number => {
                    // Todo: Unwrap for now, this is a quick first implementation
                    num_stack.push(token.value.parse().unwrap());
                }
                Tokens::Op(Operator::Plus) => {}
                Tokens::Op(Operator::Times) => {}
                Tokens::Op(_) => {}
                _ => return,
            }
        }
    }
}
