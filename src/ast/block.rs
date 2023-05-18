use super::{variable::VarData, AstNodeType};

#[derive(Debug, PartialEq)]
pub struct Block {
    pub body: Vec<AstNodeType>,
    pub line: usize,
}

impl Block {
    pub fn insert_node(&mut self, node: AstNodeType) {
        self.body.push(node.into());
    }

    pub fn scope_search(&self, name: &str) -> Option<&AstNodeType> {
        for token in &self.body {
            match &token {
                AstNodeType::Function(func) => {
                    if func.name == name {
                        let token = token;
                        return Some(token);
                    }
                }
                AstNodeType::Variable(var) => {
                    if var.get_name() == Some(name) {
                        let token = token;
                        return Some(token);
                    }
                }
                _ => continue,
            }
        }

        None
    }

    pub fn mut_scope_search(&mut self, name: &str) -> Option<&mut AstNodeType> {
        for token in &mut self.body {
            match &token {
                AstNodeType::Function(func) => {
                    if func.name == name {
                        let token = token;
                        return Some(token.as_mut());
                    }
                }
                AstNodeType::Variable(var) => {
                    if var.get_name() == Some(name) {
                        let token = token;
                        return Some(token.as_mut());
                    }
                }
                _ => continue,
            }
        }
        return None;
    }
}
