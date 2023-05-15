use super::block::Block;

pub struct Function {
    pub name: String,
    pub block: Block,
    pub line: u32,
}
