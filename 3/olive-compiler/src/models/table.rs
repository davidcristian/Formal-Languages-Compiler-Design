use super::hash_map::{HashMap, HashMapIter};

pub struct Table<K> {
    table: HashMap<K, usize>,
    current_index: usize,
}

#[allow(dead_code)]
impl<K> Table<K>
where
    K: std::fmt::Debug + Clone + Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            current_index: 1,
        }
    }

    pub fn size(&self) -> usize {
        self.table.size()
    }

    pub fn put(&mut self, key: K) -> usize {
        self.table.insert(key, self.current_index);
        self.current_index += 1;

        self.current_index - 1
    }

    pub fn insert(&mut self, key: K, value: usize) {
        self.table.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&usize> {
        self.table.get(key)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.table.contains(key)
    }

    pub fn remove(&mut self, key: &K) {
        self.table.remove(key)
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }

    pub fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            current_index: self.current_index,
        }
    }

    pub fn to_string(&self) -> String {
        self.table.to_string()
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
