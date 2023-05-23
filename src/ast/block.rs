use super::{variable::VarData, Expr};

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub body: Vec<Expr>,
    pub line: usize,
}

impl Block {
    pub fn insert_node(&mut self, node: Expr) {
        self.body.push(node.into());
    }

    pub fn scope_search(&self, name: &str) -> Option<&Expr> {
        for token in &self.body {
            match &token {
                Expr::Function(func) => {
                    if func.name == name {
                        let token = token;
                        return Some(token);
                    }
                }
                Expr::Variable(var) => {
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

    pub fn mut_scope_search(&mut self, name: &str) -> Option<&mut Expr> {
        for token in &mut self.body {
            match &token {
                Expr::Function(func) => {
                    if func.name == name {
                        let token = token;
                        return Some(token.as_mut());
                    }
                }
                Expr::Variable(var) => {
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
