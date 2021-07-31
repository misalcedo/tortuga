use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Process {
    pub identifier: Identifier,
    pub intents: Vec<Intent>,
}

impl Process {
    pub fn new(identifier: Identifier) -> Self {
        Process {
            identifier,
            intents: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    pub path: Vec<String>,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.join("/"))
    }
}

impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier {
            path: vec![name.to_string()],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProcessHandle {
    pub name: String,
}

impl ProcessHandle {
    pub fn new(name: &str) -> Self {
        ProcessHandle {
            name: name.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    pub signature: Signature,
    pub expression: Expression,
}

impl Default for Intent {
    fn default() -> Self {
        Intent {
            signature: Signature::default(),
            expression: Expression::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub fields: Vec<Field>,
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            fields: Vec::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: FieldName,
    pub kind: FieldKind,
}

impl Field {
    pub fn new(name: FieldName, kind: FieldKind) -> Self {
        Field { name, kind }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FieldName(String);

impl FieldName {
    pub fn new(name: &str) -> Self {
        FieldName(name.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FieldKind {
    Number,
    ProcessHandle,
    Tail,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Expression {
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
pub enum Instruction {
    SendToField(SendToField),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SendToField {
    recipient: FieldName,
    message: Message,
}

impl SendToField {
    pub fn new(recipient: FieldName, message: Message) -> Self {
        SendToField { recipient, message }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub data: Vec<Datum>,
}

impl Default for Message {
    fn default() -> Self {
        Message { data: Vec::new() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Datum {
    Number(Number),
    SelfHandle,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Number {
    whole: u128,
    fractional: Option<u128>,
}

impl Number {
    pub fn new(whole: u128, fractional: Option<u128>) -> Self {
        Number { whole, fractional }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml() {
        let mut message = Message::default();
        message.data.push(Datum::SelfHandle);
        message.data.push(Datum::Number(Number::new(42, None)));

        let sender = FieldName::new("sender");
        let mut intent = Intent::default();
        intent.signature.fields.push(Field::new(sender.clone(), FieldKind::ProcessHandle));
        intent.signature.fields.push(Field::new(FieldName::new("value"), FieldKind::Number));
        intent.expression.instructions.push(Instruction::SendToField(SendToField::new(sender.clone(), message)));

        let mut example = Process::new(Identifier::new("example"));
        example.intents.push(intent);

        let string = serde_yaml::to_string(&example).unwrap();
        let process: Process = serde_yaml::from_str(string.as_str()).unwrap();

        assert_eq!(process, example);
    }
}
