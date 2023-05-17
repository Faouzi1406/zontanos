use super::block::Block;

#[derive(Debug)]
pub enum ReturnTypes {
    I8,
    U8,
    I32,
    F32,
    Char,
    String,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub block: Block,
    pub return_type: ReturnTypes,
    pub line: u32,
}
