use crate::reference::Reference;
use std::collections::HashMap;
use std::vec::Drain;

/// Allows actors in the system to send each other messages.
pub struct Broker {
    mailboxes: HashMap<Reference, Vec<Vec<u8>>>,
}

impl Broker {
    pub fn new() -> Broker {
        Broker {
            mailboxes: HashMap::new(),
        }
    }

    pub fn send(&mut self, to: Reference, message: &[u8]) {
        let mailbox = self.mailboxes.entry(to).or_insert_with(|| Vec::new());

        mailbox.push(message.to_vec());
    }

    pub fn read(&mut self, to: Reference) -> Drain<Vec<u8>> {
        let mailbox = self.mailboxes.entry(to).or_insert_with(|| Vec::new());

        mailbox.drain(..)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{forget, size_of};

    #[test]
    fn send() {
        let mut dc = Broker::new();
        let to = Reference::new();
        let mut message = vec![42];

        let ptr = message.as_mut_ptr() as *mut u8;
        let length = message.len() * size_of::<u32>();

        forget(numbers);

        let message = unsafe { Vec::from_raw_parts(ptr, length, length) };

        dc.send(to, &message);

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
