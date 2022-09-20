use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct IndexedSet<Key, Value = Key> {
    instances: Vec<Value>,
    indices: HashMap<Key, usize>,
}

impl<K, V: PartialEq> PartialEq for IndexedSet<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.instances.eq(&other.instances)
    }

    fn ne(&self, other: &Self) -> bool {
        self.instances.ne(&other.instances)
    }
}

impl<K, V> Default for IndexedSet<K, V> {
    fn default() -> Self {
        IndexedSet {
            instances: Vec::default(),
            indices: HashMap::default(),
        }
    }
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

impl<K, const N: usize> From<[K; N]> for IndexedSet<K>
where
    K: Eq + Hash + Clone,
{
    fn from(array: [K; N]) -> Self {
        let mut set = IndexedSet {
            instances: Vec::with_capacity(N),
            indices: HashMap::with_capacity(N),
        };

        for (index, key) in array.into_iter().enumerate() {
            set.instances.push(key.clone());
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

    pub fn insert_with<F>(&mut self, key: K, f: F) -> usize
    where
        F: FnOnce(usize) -> V,
    {
        match self.indices.get(&key) {
            Some(index) => *index,
            None => {
                let index = self.instances.len();
                let value = f(index);

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

    pub fn get_mut(&mut self, index: usize) -> Option<&mut V> {
        self.instances.get_mut(index)
    }

    pub fn contains<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self.indices.get(key) {
            None => false,
            Some(index) => self.instances.get(*index).is_some(),
        }
    }

    pub fn lookup<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.indices.get(key)?;
        self.instances.get(*index)
    }

    pub fn lookup_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let index = self.indices.get(key)?;
        self.instances.get_mut(*index)
    }
}

impl<K, V> From<IndexedSet<K, V>> for Vec<V> {
    fn from(values: IndexedSet<K, V>) -> Self {
        values.instances
    }
}
