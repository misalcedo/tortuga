use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reference {
    global: Uuid,
}

impl Reference {
    pub fn new() -> Reference {
        Reference {
            global: Uuid::new_v4(),
        }
    }
}
