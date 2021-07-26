use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Node {
    name: Name,
    children: Vec<ChildDeclaration>,
    intentions: Vec<Intention>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Name {
    identifier: Uri,
}

impl Default for Name {
    fn default() -> Self {
        Name {
            identifier: Uri::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Uri {
    scheme: String,
    host: String,
    port: u16,
    path: PathBuf,
}

impl Default for Uri {
    fn default() -> Self {
        Uri {
            scheme: String::default(),
            host: String::default(),
            port: u16::default(),
            path: PathBuf::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct ChildDeclaration {
    shorthand: Handle,
    identifier: Uri,
}

impl Default for ChildDeclaration {
    fn default() -> Self {
        ChildDeclaration {
            shorthand: Handle::default(),
            identifier: Uri::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Handle {
    name: String,
}

impl Default for Handle {
    fn default() -> Self {
        Handle {
            name: String::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Intention {
    signature: Signature,
    expression: Expression,
}

impl Default for Intention {
    fn default() -> Self {
        Intention {
            signature: Signature::default(),
            expression: Expression::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Signature {
    fields: Vec<Kind>,
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            fields: Vec::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum Kind {
    Number,
    BitVector(usize),
    ResourceName,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Expression {
    instructions: Vec<Instruction>,
}

impl Default for Expression {
    fn default() -> Self {
        Expression {
            instructions: Vec::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct ChildName {
    name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum Instruction {
    Send,
    Respond,
}

impl Node {
    pub fn new() -> Self {
        Node {
            name: Name::default(),
            children: Vec::new(),
            intentions: Vec::new(),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml() {
        let mut node = Node::default();
        node.name.identifier.path.push("/ping");

        let mut child = ChildDeclaration::default();
        child.shorthand.name = "pong".to_string();
        child.identifier.path.push("/pong");
        node.children.push(child);

        let mut intention = Intention::default();
        node.intentions.push(intention);

        println!("{}", serde_yaml::to_string(&node).unwrap());
    }
}
