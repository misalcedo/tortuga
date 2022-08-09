use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Default)]
pub struct Constants<Constant, Key = Constant> {
    instances: Vec<Constant>,
    indices: HashMap<Key, usize>,
}

impl<C, K> Constants<C, K>
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
}

impl<C, K> From<Constants<C, K>> for Vec<C> {
    fn from(constants: Constants<C, K>) -> Self {
        constants.instances
    }
}
