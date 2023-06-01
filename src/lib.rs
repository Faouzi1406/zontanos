//! The main compiler for **Zontanos**.
//!
//! It takes any zontanos source code and compiles it into machine code.

/// The main Ast of the language. It contains the structure of the Ast(Abstract syntax tree) that
/// gets generated after parsing.
pub mod ast;
pub mod codegen;
pub mod codegen_v2;
pub mod parser_v2;
/// The main parser of the language, it parses any given valid Zontanos source code into its
/// Ast.
pub mod zon_parser;
