mod reference;

use std::collections::HashMap;
use std::vec::Drain;
use self::reference::Reference;

/// Allows actors in the system to send each other messages.
struct DistributionCenter {
    mailboxes: HashMap<Reference, Vec<u32>>
}

impl DistributionCenter {
    fn new() -> DistributionCenter {
        DistributionCenter {
            mailboxes: HashMap::new()
        }
    }

    fn send(&mut self, to: Reference, message: u32) {
        let mailbox = self.mailboxes.entry(to).or_insert_with(|| Vec::new());

        mailbox.push(message);
    }

    fn read(&mut self, to: Reference) -> Drain<u32> {
        let mailbox = self.mailboxes.entry(to).or_insert_with(|| Vec::new());
        
        mailbox.drain(..)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send() {
        let mut dc = DistributionCenter::new();
        let to = Reference::new();
        let message = 42;

        dc.send(to, message);
        
        let mailbox = dc.read(to);
        let messages: Vec<u32> = mailbox.collect();

        assert_eq!(messages, vec![message]);
    }

    #[test]
    fn read_empty() {
        let mut dc = DistributionCenter::new();
        let to = Reference::new();

        let mailbox = dc.read(to);
        let messages: Vec<u32> = mailbox.collect();

        assert_eq!(messages, vec![]);
    }
}