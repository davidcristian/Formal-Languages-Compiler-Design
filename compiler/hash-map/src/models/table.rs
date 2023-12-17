use super::hash_map::{HashMap, HashMapIter};

pub struct Table<V> {
    table: HashMap<usize, V>,
    current_index: usize,
}

impl<V> Table<V>
where
    V: PartialEq,
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

    pub fn put(&mut self, value: V) -> usize {
        // check if the value already exists, if yes return the key
        for (key, val) in &self.table {
            if val == &value {
                return *key;
            }
        }

        let index = self.current_index;
        self.table.insert(index, value);

        self.current_index += 1;
        index
    }

    pub fn get(&self, key: &usize) -> Option<&V> {
        self.table.get(key)
    }

    pub fn clear(&mut self) {
        self.table.clear();
        self.current_index = 1;
    }
}

// iterator implementation for the Table

impl<'a, V> Table<V> {
    // returns an iterator over the data in the table
    pub fn iter(&'a self) -> HashMapIter<'a, usize, V> {
        self.table.iter()
    }
}

impl<'a, V> IntoIterator for &'a Table<V> {
    type Item = (&'a usize, &'a V);
    type IntoIter = HashMapIter<'a, usize, V>;

    // returns an iterator over the table
    fn into_iter(self) -> Self::IntoIter {
        self.table.iter()
    }
}
