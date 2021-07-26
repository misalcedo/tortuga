use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Process {
    identifier: Uri,
    children: Vec<ChildDeclaration>,
    texts: Vec<TextDeclaration>,
    intentions: Vec<Intention>,
}

impl Default for Process {
    fn default() -> Self {
        Process {
            identifier: Uri::default(),
            children: Vec::new(),
            texts: Vec::new(),
            intentions: Vec::new(),
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
    handle: ProcessHandle,
    identifier: Uri,
}

impl Default for ChildDeclaration {
    fn default() -> Self {
        ChildDeclaration {
            handle: ProcessHandle::default(),
            identifier: Uri::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct ProcessHandle {
    name: String,
}

impl Default for ProcessHandle {
    fn default() -> Self {
        ProcessHandle {
            name: String::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct TextDeclaration {
    handle: TextHandle,
    reference: TextReference,
}

impl Default for TextDeclaration {
    fn default() -> Self {
        TextDeclaration {
            handle: TextHandle::default(),
            reference: TextReference::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct TextHandle {
    name: String,
}

impl Default for TextHandle {
    fn default() -> Self {
        TextHandle {
            name: String::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct TextReference {
    identifier: String,
}

impl Default for TextReference {
    fn default() -> Self {
        TextReference {
            identifier: String::default(),
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
    fields: Vec<Field>,
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            fields: Vec::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Field {
    name: Option<FieldName>,
    kind: FieldKind,
}

impl Default for Field {
    fn default() -> Self {
        Field {
            name: Some(FieldName::default()),
            kind: FieldKind::Tail,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct FieldName {
    name: String,
}

impl Default for FieldName {
    fn default() -> Self {
        FieldName {
            name: String::default(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum FieldKind {
    Number {
        whole: u128,
        fractional: Option<u128>,
    },
    ByteVector {
        length: usize,
    },
    ProcessHandle(ProcessHandle),
    TextHandle(TextHandle),
    AnyProcessHandle,
    Tail,
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
enum Instruction {
    SendToField {
        recipient: FieldName,
        message: Message,
    },
    SendToChild {
        recipient: ProcessHandle,
        message: Message,
    },
    SendToSelf(Message),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Message {
    data: Vec<Datum>,
}

impl Default for Message {
    fn default() -> Self {
        Message { data: Vec::new() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum Datum {
    Number {
        whole: u128,
        fractional: Option<u128>,
    },
    ByteVector {
        length: usize,
    },
    ProcessHandle(ProcessHandle),
    TextHandle(TextHandle),
    SelfHandle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml() {
        let mut ping = Process::default();
        ping.identifier.path.push("/ping");

        let mut pong = ChildDeclaration::default();
        pong.handle.name.push_str("pong");
        pong.identifier.path.push("/pong");
        ping.children.push(pong.clone());

        let mut message = TextDeclaration::default();
        message.handle.name.push_str("message");
        message.reference.identifier.push_str("ping");
        ping.texts.push(message.clone());

        let mut unknown_sender = TextDeclaration::default();
        unknown_sender.handle.name.push_str("unknown_sender");
        unknown_sender
            .reference
            .identifier
            .push_str("unknown_sender");
        ping.texts.push(unknown_sender.clone());

        let mut accept_pong = Intention::default();
        let mut pong_handle = Field::default();
        pong_handle.name = None;
        pong_handle.kind = FieldKind::ProcessHandle(pong.handle.clone());
        accept_pong.signature.fields.push(pong_handle);
        let mut data = Field::default();
        data.name.as_mut().map(|f| f.name.push_str("data"));
        data.kind = FieldKind::ByteVector { length: 5 };
        accept_pong.signature.fields.push(data);
        let mut pong_message = Message::default();
        pong_message.data.push(Datum::SelfHandle);
        pong_message
            .data
            .push(Datum::TextHandle(message.handle.clone()));
        accept_pong
            .expression
            .instructions
            .push(Instruction::SendToChild {
                recipient: pong.handle.clone(),
                message: pong_message,
            });
        ping.intentions.push(accept_pong);

        let mut accept_any_sender = Intention::default();
        let mut sender_handle = Field::default();
        sender_handle
            .name
            .as_mut()
            .map(|h| h.name.push_str("sender"));
        sender_handle.kind = FieldKind::AnyProcessHandle;
        accept_any_sender
            .signature
            .fields
            .push(sender_handle.clone());
        let mut tail = Field::default();
        tail.name = None;
        accept_any_sender.signature.fields.push(tail);
        let mut unknown_message = Message::default();
        unknown_message.data.push(Datum::SelfHandle);
        unknown_message
            .data
            .push(Datum::TextHandle(unknown_sender.handle.clone()));
        accept_any_sender
            .expression
            .instructions
            .push(Instruction::SendToField {
                recipient: sender_handle.name.clone().unwrap(),
                message: unknown_message,
            });
        ping.intentions.push(accept_any_sender);

        let string = serde_yaml::to_string(&ping).unwrap();
        let process: Process = serde_yaml::from_str(string.as_str()).unwrap();

        assert_eq!(process, ping);
    }
}
