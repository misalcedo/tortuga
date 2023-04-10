use uuid::Uuid;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Identifier(u128);

impl Default for Identifier {
    fn default() -> Self {
        Identifier::new(Uuid::new_v4())
    }
}

impl Identifier {
    fn new(value: Uuid) -> Self {
        Identifier(value.to_u128_le())
    }
}

impl<Name> From<Name> for Identifier
where
    Name: AsRef<str>,
{
    fn from(value: Name) -> Self {
        Identifier::new(Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            value.as_ref().as_bytes(),
        ))
    }
}
