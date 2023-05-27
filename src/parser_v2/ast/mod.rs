//! The Abstract Syntax Tree structure of Zontanos
#![allow(unused)]

use crate::zon_parser::lexer::Operator;

pub mod types_from_str;
pub mod types_match;

pub struct Ast {
    pub r#type: NodeTypes,
    pub body: Vec<Node>,
}

/// [`Node`]
/// A Node in the abstract syntax tree;
///
/// **Line** the line of where the node is located in source code
/// **right** The values/nodes right of the Node
/// **left** The values/nodes left of the Node
/// **NodeTypes** The type of a Node
#[derive(Debug)]
pub struct Node {
    pub node_type: NodeTypes,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub line: usize,
}

/// [`Function`]
/// A Function holds the general structure of the Function;
///
/// **ident** Identifier/name of the function
/// **body** The body `{<body>}` of a function
/// **returns** The type the function returns
#[derive(Debug)]
pub struct Function {
    pub ident: Ident,
    pub body: Box<Vec<Node>>,
    pub paramaters: Vec<Paramater>,
    pub returns: Type,
}

/// [`Variable`]
/// A Variable holds the structure of a variable;
///
/// **ident** identifier/name of variable
/// **var_type** the type of the variable
#[derive(Debug)]
pub struct Variable {
    pub ident: Ident,
    pub var_type: Type,
}

/// [`Assignment`]
/// Assigment to a variable that exists
///
/// **assigns_to** the ident of the variable being reassigned/assigned to
#[derive(Debug)]
pub struct Assignment {
    pub assigns_to: Ident,
}

/// [`Type`]
/// Type of value
///
/// **r#type** the type it is
/// **generics** all of the generic values if any, will be empty if there are none
#[derive(Clone, Debug)]
pub struct Type {
    pub r#type: Types,
    pub generics: Vec<Type>,
}

/// [`Paramater`]
/// Function paramater containing both the type and the name
///
/// **r#type** the type of paramater
/// **generics** the name of para
#[derive(Debug)]
pub struct Paramater {
    r#type: Type,
    name: String,
}

/// [`Ident`]
/// Identifier of a value
///
/// **r#name** the name of that value
#[derive(Debug)]
pub struct Ident {
    pub name: String,
}

/// [`Ident`]
/// Identifier of a value
///
/// **r#name** the name of that value
/// **value** the value
#[derive(Debug)]
pub struct Value {
    pub r#type: TypeValues,
}

/// [`FunctionCall`]
/// Call to Function
///
/// **r#calls_to** the Ident of the function being called upon
/// **arguments** all the arguments found in the function call
#[derive(Debug)]
pub struct FunctionCall {
    pub calls_to: Ident,
    pub arguments: Vec<Value>,
}

/// [`Types`]
/// All current types in a language
#[derive(Clone, PartialEq, Debug)]
pub enum Types {
    I8,
    U8,
    I32,
    F32,
    Char,
    String,
    Array,
    Ident,
    UnknownType(String),
}

/// [`Types`]
/// All current type + values in the language
#[derive(Debug, PartialEq)]
pub enum TypeValues {
    I8(i8),
    U8(u8),
    I32(i32),
    F32(f32),
    Char(char),
    String(String),
    Array(Vec<TypeValues>),
    Identifier(String),
    None
}

/// [`NodeTypes`]
/// Type of nodes there are
#[derive(Debug)]
pub enum NodeTypes {
    Program,
    Function(Function),
    Variable(Variable),
    Assignment(Assignment),
    Operator(Operator),
    Value(Value),
    FunctionCall(FunctionCall),
}

impl Node {
    pub fn new(node_type: NodeTypes, line: usize) -> Self {
        Self {
            node_type,
            left: None,
            right: None,
            line,
        }
    }
}
