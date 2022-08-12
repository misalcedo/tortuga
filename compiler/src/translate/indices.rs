use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Default)]
pub struct IndexedSet<Constant, Key = Constant> {
    instances: Vec<Constant>,
    indices: HashMap<Key, usize>,
}

impl<C, K> IndexedSet<C, K>
where
    K: Hash + Eq,
{
    pub fn insert(&mut self, constant: C, key: K) -> usize {
        match self.indices.get(&key) {
            Some(index) => *index,
            None => {
                let index = self.instances.len();

                self.instances.push(constant);
                self.indices.insert(key, index);

                index
            }
        }
    }

    pub fn add(&mut self, constant: C, key: K) -> usize {
        match self.indices.get(&key) {
            Some(index) => *index,
            None => {
                let index = self.instances.len();

                self.instances.push(constant);
                self.indices.insert(key, index);

                index
            }
        }
    }

    pub fn get(&mut self, index: usize) -> Option<&C> {
        self.instances.get(index)
    }
}

impl<C, K> From<IndexedSet<C, K>> for Vec<C> {
    fn from(constants: IndexedSet<C, K>) -> Self {
        constants.instances
    }
}
