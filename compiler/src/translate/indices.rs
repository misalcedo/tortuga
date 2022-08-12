use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug, Default)]
pub struct IndexedSet<Key, Value = Key> {
    instances: Vec<Value>,
    indices: HashMap<Key, usize>,
}

impl<K, V, const N: usize> From<[(K, V); N]> for IndexedSet<K, V>
where
    K: Eq + Hash,
{
    fn from(array: [(K, V); N]) -> Self {
        let mut set = IndexedSet {
            instances: Vec::with_capacity(N),
            indices: HashMap::with_capacity(N),
        };

        for (index, (key, value)) in array.into_iter().enumerate() {
            set.instances.push(value);
            set.indices.insert(key, index);
        }

        set
    }
}

impl<K, V> IndexedSet<K, V>
where
    K: Hash + Eq,
{
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    pub fn insert(&mut self, key: K, value: V) -> usize {
        match self.indices.get(&key) {
            Some(index) => *index,
            None => {
                let index = self.instances.len();

                self.instances.push(value);
                self.indices.insert(key, index);

                index
            }
        }
    }

    pub fn add(&mut self, value: V) -> usize {
        let index = self.instances.len();

        self.instances.push(value);

        index
    }

    pub fn get(&self, index: usize) -> Option<&V> {
        self.instances.get(index)
    }

    pub fn lookup(&self, key: &K) -> Option<&V> {
        let index = self.indices.get(key)?;
        self.instances.get(*index)
    }
}

impl<K, V> From<IndexedSet<K, V>> for Vec<V> {
    fn from(values: IndexedSet<K, V>) -> Self {
        values.instances
    }
}
