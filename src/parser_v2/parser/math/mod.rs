use std::fmt::Debug;

use crate::zon_parser::lexer::{Operator, Tokens};

// 1 + 1 = 2
// 2 * 3 + 2 = 8
// 1 + 1 * 3 = 4
use super::Parser;

pub enum MathOperations {
    Add(MathExpr, MathExpr),
    Minus(MathExpr, MathExpr),
    Substract(MathExpr, MathExpr),
    Multiply(MathExpr, MathExpr),
    Divide(MathExpr, MathExpr),
}

pub struct MathExpr {
    pub value: i32,
    pub left: Box<MathExpr>,
    pub right: Box<MathExpr>,
}

impl Parser {
    fn parse_math(&mut self) {
        while let Some(token) = self.next() {
            if let Tokens::Op(op) = token.token_type {
                match op {
                    Operator::Times => {}
                    Operator::Plus => {}
                    Operator::Min => {}
                    _ => return,
                }
            }
        }
    }
}

pub struct Stack<T> {
    pub value: Option<T>,
    pub next: Option<Box<Stack<T>>>,
}

impl<T> Stack<T> {
    pub fn init() -> Self {
        Self {
            value: None,
            next: None,
        }
    }

    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
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
        self.next = Some(Box::from(Self::new(value)));
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Clone,
    {
        if self.next.is_some() {
            return self.next.as_mut().unwrap().pop();
        }
        let value = self.value.clone()?;
        self.value = None;
        return Some(value);
    }
}

impl<T> Debug for Stack<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("stack")
            .field("value", &self.value)
            .field("next", &self.next)
            .finish()
    }
}
