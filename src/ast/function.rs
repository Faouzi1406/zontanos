use super::{block::Block, types::MarkerTypes};

#[derive(Debug, PartialEq, Clone)]
pub struct Paramater {
    pub name: String,
    pub paramater_type: MarkerTypes,
}

impl Paramater {
    /// Creates a new paramater with a name and type
    pub fn new(name: &str, paramater_type: MarkerTypes) -> Self {
        Paramater {
            name: name.to_string(),
            paramater_type,
        }
    }

    pub fn set_type(&mut self, paramater_type: MarkerTypes) -> Result<(), String> {
        if self.paramater_type != MarkerTypes::None {
            return Err(format!("This paramater already has a type"));
        }
        if self.name != "" {
            return Err(format!(
                "Tried to assign type to paramater that already has a name."
            ));
        }
        self.paramater_type = paramater_type;
        Ok(())
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), String> {
        if self.name != "" {
            return Err(format!("This paramater already has a name"));
        }
        if self.paramater_type == MarkerTypes::None {
            return Err(format!("A type was assigned before a name was set"));
        }
        self.name = name.to_string();
        return Ok(());
    }

    /// Checks if a paramater is a valid one,
    /// A valid paramater is a paramater that has both a type and name
    pub fn is_valid(&mut self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("paramater doesn't have a name".to_string());
        }
        if self.paramater_type == MarkerTypes::None {
            return Err("paramater doesn't have a type".to_string());
        }
        return Ok(());
    }

    /// Sets the name to and empty string and type to MarketType::None
    pub fn clear(&mut self) {
        self.name.clear();
        self.paramater_type = MarkerTypes::None;
    }
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub block: Block,
    pub return_type: MarkerTypes,
    pub params: Vec<Paramater>,
    pub line: usize,
}
