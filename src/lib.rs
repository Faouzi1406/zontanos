//! The main compiler for **Zontanos**.
//!
//! It takes any zontanos source code and compiles it into machine code.

pub mod codegen;

/// The main parser of the language, it parses any given valid Zontanos source code into its
/// Ast.
pub mod zon_parser;

/// The main Ast of the language. It contains the structure of the Ast(Abstract syntax tree) that
/// gets generated after parsing.
pub mod ast;
mod parser_v2;
