//! The Abstract Syntax Tree structure of Zontanos
#![allow(unused)]

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
pub struct Node {
    node_type: NodeTypes,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    line: usize,
}

/// [`Function`]
/// A Function holds the general structure of the Function;
///
/// **ident** Identifier/name of the function
/// **body** The body `{<body>}` of a function
/// **returns** The type the function returns
pub struct Function {
    pub ident: Ident,
    pub body: Box<Vec<Node>>,
    pub returns: Type,
}

/// [`Variable`]
/// A Variable holds the structure of a variable;
///
/// **ident** identifier/name of variable
/// **var_type** the type of the variable
pub struct Variable {
    pub ident: Ident,
    pub var_type: Type,
}

/// [`Assignment`]
/// Assigment to a variable that exists
///
/// **assigns_to** the ident of the variable being reassigned/assigned to
pub struct Assignment {
    pub assigns_to: Ident,
}

/// [`Type`]
/// Type of value
///
/// **r#type** the type it is
/// **generics** all of the generic values if any, will be empty if there are none
pub struct Type {
    pub r#type: Types,
    pub generics: Vec<Type>,
}

/// [`Ident`]
/// Identifier of a value
///
/// **r#name** the name of that value
pub struct Ident {
    pub name: String,
}

/// [`Types`]
/// All current types in a language
pub enum Types {
    I8,
    U8,
    I32,
    F32,
    Char,
    String,
    Array,
    UnknownType(String)
}

/// [`NodeTypes`]
/// Type of nodes there are
pub enum NodeTypes {
    Program,
    Function(Function),
    Variable(Variable),
    Assignment(Assignment),
}
