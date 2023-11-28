use super::hash_map::{HashMap, HashMapIter};

pub struct Table<K> {
    table: HashMap<K, usize>,
    current_index: usize,
}

impl<K> Table<K>
where
    K: Clone + Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            current_index: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn put(&mut self, key: K) -> usize {
        if let Some(value) = self.table.get(&key) {
            return *value;
        }

        let index = self.current_index;
        self.table.insert(key, index);

        self.current_index += 1;
        index
    }

    pub fn insert(&mut self, key: K, value: usize) {
        self.table.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&usize> {
        self.table.get(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.table.contains_key(key)
    }

    pub fn remove(&mut self, key: &K) {
        self.table.remove(key)
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }
}

// iterator implementation for the Table

impl<'a, K> Table<K> {
    // returns an iterator over the data in the table
    pub fn iter(&'a self) -> HashMapIter<'a, K, usize> {
        self.table.iter()
    }
}

impl<'a, K> IntoIterator for &'a Table<K> {
    type Item = (&'a K, &'a usize);
    type IntoIter = HashMapIter<'a, K, usize>;

    // returns an iterator over the table
    fn into_iter(self) -> Self::IntoIter {
        self.table.iter()
    }
}
