use crate::reference::Reference;
use std::collections::HashMap;
use std::vec::Drain;

/// Allows actors in the system to send each other messages.
pub struct Broker {
    mailboxes: HashMap<Reference, Vec<u32>>,
}

impl Broker {
    pub fn new() -> Broker {
        Broker {
            mailboxes: HashMap::new(),
        }
    }

    pub fn send(&mut self, to: Reference, message: u32) {
        let mailbox = self.mailboxes.entry(to).or_insert_with(|| Vec::new());

        mailbox.push(message);
    }

    pub fn read(&mut self, to: Reference) -> Drain<u32> {
        let mailbox = self.mailboxes.entry(to).or_insert_with(|| Vec::new());

        mailbox.drain(..)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send() {
        let mut dc = Broker::new();
        let to = Reference::new();
        let message = 42;

        dc.send(to, message);

        let mailbox = dc.read(to);
        let messages: Vec<u32> = mailbox.collect();

        assert_eq!(messages, vec![message]);
    }

    #[test]
    fn read_empty() {
        let mut dc = Broker::new();
        let to = Reference::new();

        let mailbox = dc.read(to);
        let messages: Vec<u32> = mailbox.collect();

        assert_eq!(messages, vec![]);
    }
}
