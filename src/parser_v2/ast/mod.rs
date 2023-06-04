//! The Abstract Syntax Tree structure of Zontanos
#![allow(unused)]

use crate::{ast::variable, zon_parser::lexer::Operator};

pub mod types_from_str;
pub mod types_match;

#[derive(Debug)]
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
    pub body: Vec<Node>,
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
    pub is_array: bool,
    pub size: u32,
    pub generics: Vec<Type>,
}

/// [`Paramater`]
/// Function paramater containing both the type and the name
///
/// **r#type** the type of paramater
/// **ident** the identifier of paramater
#[derive(Debug)]
pub struct Paramater {
    pub r#type: Type,
    pub ident: Ident,
}

/// [`Ident`]
/// Identifier of a value
///
/// **r#name** the name of that value
#[derive(Debug, PartialEq, Clone)]
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
    pub value: TypeValues,
}

/// [`FunctionCall`]
/// Call to Function
///
/// **r#calls_to** the Ident of the function being called upon
/// **arguments** all the arguments found in the function call
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub calls_to: Ident,
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
    // Should only be used if the type can not be known during parsing.
    None,
    UnknownType(String),
}

/// [`Types`]
/// All current type + values in the language
#[derive(Debug, PartialEq, Clone)]
pub enum TypeValues {
    I8(i8),
    U8(u8),
    I32(i32),
    F32(f32),
    Char(char),
    String(String),
    FunctionCall(FunctionCall),
    Array(Vec<TypeValues>),
    Identifier(String),
    NoneVal(String),
    None,
}

/// [`NodeTypes`]
/// Type of nodes there are
#[derive(Debug)]
pub enum NodeTypes {
    Program,
    Block(Vec<Node>),
    Function(Function),
    Variable(Variable),
    Assignment(Assignment),
    Operator(Operator),
    Value(Value),
    FunctionCall(FunctionCall),
    Arguments(Vec<Value>),
    Return
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

    pub fn variable(variable: Variable, left_operator: Operator, line: usize) -> Node {
        let node = Node {
            node_type: NodeTypes::Variable(variable),
            left: Some(Box::from(Node::new(
                NodeTypes::Operator(left_operator),
                line,
            ))),
            right: None,
            line,
        };
        node
    }

    pub fn fn_call(function_call: FunctionCall, arguments: NodeTypes, line: usize) -> Node {
        let node_type = NodeTypes::FunctionCall(function_call);
        let arguments = Node::new(arguments, line);
        Node {
            node_type,
            left: Some(Box::from(arguments)),
            right: None,
            line,
        }
    }
}

impl Type {
    pub fn none_type() -> Self {
        Self {
            r#type: Types::None,
            is_array: false,
            size: 0,
            generics: Vec::new(),
        }
    }
}

impl From<TypeValues> for Value {
    fn from(value: TypeValues) -> Self {
        Self { value }
    }
}

impl FunctionCall {
    /// Gets the arguments of a function call node, expecting them to be on the left node, if they
    /// are not there None is returned.
    ///
    /// **Important to note: If the arguments are placed incorrectly during parsing it will result in this function always returning None, 
    /// it is therefore pivotal that the arguments of a function call are always placed in the
    /// left node.**
    pub fn get_args<'ctx>(&self, from: &'ctx Node) -> Option<&'ctx Vec<Value>> {
        let NodeTypes::Arguments(args) = &from.left.as_ref()?.node_type else {
            return None;
        };
        Some(args)
    }
}
