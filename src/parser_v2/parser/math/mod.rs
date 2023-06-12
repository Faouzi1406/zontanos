use crate::zon_parser::lexer::{Operator, Tokens};
use std::fmt::Debug;

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

#[derive(Debug)]
pub struct Stack<'a, T> {
    head: Option<Node<'a, T>>,
}

#[derive(Debug, Clone)]
struct Node<'a, T> {
    next: Option<Box<Node<'a, T>>>,
    value: &'a T,
}

impl<'a, T> Stack<'a, T>
where
    T: Debug + Clone,
{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_front(&mut self, value: &'a T) {
        let mut node = Node::new(value);

        let Some(head) = &self.head else {
            self.head = Some(node);
            return
        };

        node.next = Some(Box::new(head.clone()));
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<&T> {
        let head =  self.head.clone()?;
        if let Some(next) = head.next {
            self.head = Some(*next);
        } else {
            self.head = None;
        }
        Some(head.value)
    }
}

impl<'a, T> Node<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self { next: None, value }
    }
}
