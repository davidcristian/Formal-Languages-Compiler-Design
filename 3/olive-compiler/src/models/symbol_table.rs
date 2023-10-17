use super::hash_table::HashTable;

pub struct SymbolTable<K> {
    table: HashTable<K, usize>,
    current_index: usize,
}

#[allow(dead_code)]
impl<K> SymbolTable<K>
where
    K: std::fmt::Debug + Clone + Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        SymbolTable {
            table: HashTable::new(),
            current_index: 1,
        }
    }

    pub fn size(&self) -> usize {
        self.table.size()
    }

    pub fn put(&mut self, key: K) {
        self.table.put(key, self.current_index);
        self.current_index += 1;
    }

    pub fn get(&self, key: &K) -> Option<&usize> {
        self.table.get(key)
    }

    pub fn remove(&mut self, key: &K) {
        self.table.remove(key)
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }

    pub fn display(&self) {
        self.table.display();
    }
}
