use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    name: String
}

impl User {
    pub fn new(name: &str) -> Self {
        User { name: name.to_owned() }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}