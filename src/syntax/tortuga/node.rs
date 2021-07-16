use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node {}

impl Node {
    pub fn new() -> Node {
        Node {}
    }
}
