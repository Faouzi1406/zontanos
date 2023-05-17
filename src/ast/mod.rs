//! src/ast
//!
//! This will contain the Ast of the language.
//! The Ast will be used to compile the language into llvm-ir;
#![allow(dead_code)]

use self::function::Function;
pub mod block;
pub mod function;
pub mod function_call;
pub mod types;
pub mod variable;

#[derive(Debug)]
pub enum AstNodeType {
    Variable(variable::Variable),
    Function(function::Function),
    Block(block::Block),
    Program,
}

impl AstNodeType {
    
}

impl AstNodeType {
    
}

#[derive(Debug)]
pub struct Ast {
    ast_type: AstNodeType,
    body: Vec<AstNodeType>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            ast_type: AstNodeType::Program,
            body: Vec::new(),
        }
    }

    pub fn insert_node(&mut self, ast_type: AstNodeType) {
        self.body.push(ast_type);
    }

    pub fn find_function(&self, function_name: String) -> Option<&Function> {
        for node in self.body.iter() {
            if let AstNodeType::Function(func) = node {
                if func.name == function_name {
                    return Some(func);
                }
            }
        }
        None
    }

    pub fn find_function_mut(&mut self, function_name: String) -> Option<&Function> {
        self.find_function(function_name)
    }
}

impl From<AstNodeType> for Ast {
    fn from(ast_type: AstNodeType) -> Self {
        Self {
            ast_type,
            body: Vec::new(),
        }
    }
}
