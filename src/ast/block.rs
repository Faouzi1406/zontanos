use super::{Ast, AstNodeType};

#[derive(Debug)]
pub struct Block {
    pub body: Vec<Ast>,
    pub line: u32,
}

impl Block {
    pub fn insert_node(&mut self, node: AstNodeType) {
        self.body.push(node.into());
    }
}
