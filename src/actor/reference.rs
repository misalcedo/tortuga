use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reference(u128);

impl Reference {
    /// Creates a new unparented actor reference.
    pub fn new() -> Reference {
        let uuid = Uuid::new_v4();

        Reference(uuid.as_u128())
    }

    /// Creates a new parented and named actor reference.
    pub fn new_parented(parent: &Reference, name: &str) -> Reference {
        let parent = Uuid::from_u128(parent.0);
        let uuid = Uuid::new_v5(&parent, name.as_bytes());

        Reference(uuid.as_u128())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_unparented() {
        assert_ne!(Reference::new(), Reference::new())
    }

    #[test]
    fn new_parented() {
        let parent = Reference::new();

        assert_eq!(Reference::new_parented(&parent, "a"), Reference::new_parented(&parent, "a"));
        assert_ne!(Reference::new_parented(&parent, "a"), Reference::new_parented(&parent, "b"));
    }
}