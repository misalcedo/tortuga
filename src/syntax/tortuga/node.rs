use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node {}

impl Node {
    pub fn new() -> Self {
        Node {}
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}
