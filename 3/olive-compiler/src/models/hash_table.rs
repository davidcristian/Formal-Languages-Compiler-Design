use std::hash::Hasher;

const INITIAL_CAPACITY: usize = 16;
const RESIZE_FACTOR: usize = 2;
const LOAD_FACTOR: f64 = 0.75;

#[derive(Clone)]
struct Entry<K, V> {
    key: K,
    value: V,
    deleted: bool,
}

pub struct HashTable<K, V> {
    table: Vec<Option<Entry<K, V>>>,
    capacity: usize,
    size: usize,
}

#[allow(dead_code)]
impl<K, V> HashTable<K, V>
where
    K: std::fmt::Debug + Clone + Eq + std::hash::Hash,
    V: std::fmt::Debug + Clone,
{
    // creates a new empty hash table
    pub fn new() -> Self {
        Self {
            table: vec![None; INITIAL_CAPACITY],
            capacity: INITIAL_CAPACITY,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // first hash function
    fn hash1(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    // second hash function
    fn hash2(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        1 + ((hasher.finish() as usize) % (self.capacity - 1))
    }

    // inserts a key-value pair into the hash table
    // O(1) average time complexity (depends on hash functions and load factor)
    pub fn put(&mut self, key: K, value: V) {
        // check if we need to resize
        if (self.size as f64 / self.capacity as f64) >= LOAD_FACTOR {
            self.resize();
        }

        let mut index = self.hash1(&key);
        let hash2_value = self.hash2(&key);
        let mut increment_size = true;

        // find the next available index using double hashing
        while let Some(entry) = &self.table[index] {
            // deleted entries or entries with the same key are overwritten
            if entry.deleted {
                break;
            }
            if entry.key == key {
                increment_size = false;
                break;
            }

            index = (index + hash2_value) % self.capacity;
        }

        self.table[index] = Some(Entry {
            key,
            value,
            deleted: false,
        });

        if increment_size {
            self.size += 1;
        }
    }

    // returns the value for the given key
    // O(1) average time complexity (depends on hash functions)
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut index = self.hash1(key);
        let hash2_value = self.hash2(key);

        while let Some(entry) = &self.table[index] {
            if !entry.deleted && entry.key == *key {
                return Some(&entry.value);
            }

            index = (index + hash2_value) % self.capacity;
        }

        None
    }

    // removes the value for the given key
    // O(1) average time complexity (depends on hash functions)
    pub fn remove(&mut self, key: &K) {
        let mut index = self.hash1(key);
        let hash2_value = self.hash2(key);

        while let Some(entry) = &mut self.table[index] {
            if !entry.deleted && entry.key == *key {
                entry.deleted = true;

                self.size -= 1;
                break;
            }

            index = (index + hash2_value) % self.capacity;
        }
    }

    // clears the entire table
    // O(n) time complexity
    pub fn clear(&mut self) {
        self.table = vec![None; INITIAL_CAPACITY];
        self.capacity = INITIAL_CAPACITY;

        self.size = 0;
    }

    // display the table
    // O(n) time complexity
    pub fn display(&self) {
        for (index, entry) in self.table.iter().enumerate() {
            match entry {
                Some(e) if !e.deleted => {
                    println!("Index {}: Key: {:?}, Value: {:?}", index, e.key, e.value)
                }
                // Some(e) if e.deleted => println!("Index {}: [Deleted]", index),
                //_ => println!("Index {}: [Empty]", index),
                _ => {}
            }
        }
    }

    // resizes the table to increase its current capacity by RESIZE_FACTOR
    // O(n) time complexity
    fn resize(&mut self) {
        self.size = 0;
        self.capacity *= RESIZE_FACTOR;

        let old_table = std::mem::replace(&mut self.table, vec![None; self.capacity]);
        for entry in old_table.into_iter() {
            if let Some(e) = entry {
                if !e.deleted {
                    self.put(e.key, e.value);
                }
            }
        }
    }
}
