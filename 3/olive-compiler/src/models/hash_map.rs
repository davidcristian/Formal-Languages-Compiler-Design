use std::hash::Hasher;

const INITIAL_CAPACITY: usize = 16;
const RESIZE_FACTOR: usize = 2;
const LOAD_FACTOR: f64 = 0.6;

#[derive(Clone)]
struct Entry<K, V> {
    key: K,
    value: V,
    deleted: bool,
}

pub struct HashMap<K, V> {
    data: Vec<Option<Entry<K, V>>>,
    capacity: usize,
    size: usize,
}

#[allow(dead_code)]
impl<K, V> HashMap<K, V>
where
    K: std::fmt::Debug + Clone + Eq + std::hash::Hash,
    V: std::fmt::Debug + Clone,
{
    // creates a new empty hash map
    pub fn new() -> Self {
        Self {
            data: vec![None; INITIAL_CAPACITY],
            capacity: INITIAL_CAPACITY,
            size: 0,
        }
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

    // resizes the table to increase its current capacity by RESIZE_FACTOR
    // Complexity analysis:
    // Best: O(n)
    // Worst: O(n)
    // Average: O(n)
    fn resize(&mut self) {
        self.size = 0;
        self.capacity *= RESIZE_FACTOR;

        let old_table = std::mem::replace(&mut self.data, vec![None; self.capacity]);
        for entry in old_table.into_iter() {
            if let Some(e) = entry {
                if !e.deleted {
                    self.insert(e.key, e.value);
                }
            }
        }
    }

    // returns the current size of the hash map
    // Complexity analysis:
    // Best: O(1)
    // Worst: O(1)
    // Average: O(1)
    pub fn size(&self) -> usize {
        self.size
    }

    // inserts a key-value pair into the hash map
    // Complexity analysis:
    // Best: O(1)
    // Worst: O(n)
    // Average: O(1)
    pub fn insert(&mut self, key: K, value: V) {
        // check if we need to resize
        if (self.size as f64 / self.capacity as f64) >= LOAD_FACTOR {
            self.resize();
        }

        let mut index = self.hash1(&key);
        let hash2_value = self.hash2(&key);
        let mut increment_size = true;

        // find the next available index using double hashing
        while let Some(entry) = &self.data[index] {
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

        self.data[index] = Some(Entry {
            key,
            value,
            deleted: false,
        });

        if increment_size {
            self.size += 1;
        }
    }

    // returns the value for the given key
    // Complexity analysis:
    // Best: O(1)
    // Worst: O(n)
    // Average: O(1)
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut index = self.hash1(key);
        let hash2_value = self.hash2(key);

        while let Some(entry) = &self.data[index] {
            if !entry.deleted && entry.key == *key {
                return Some(&entry.value);
            }

            index = (index + hash2_value) % self.capacity;
        }

        None
    }

    // returns true if the hash map contains the given key
    // Complexity analysis:
    // Best: O(1)
    // Worst: O(n)
    // Average: O(1)
    pub fn contains(&self, key: &K) -> bool {
        let mut index = self.hash1(key);
        let hash2_value = self.hash2(key);

        while let Some(entry) = &self.data[index] {
            if !entry.deleted && entry.key == *key {
                return true;
            }

            index = (index + hash2_value) % self.capacity;
        }

        false
    }

    // removes the key-value pair for the given key
    // Complexity analysis:
    // Best: O(1)
    // Worst: O(n)
    // Average: O(1)
    pub fn remove(&mut self, key: &K) {
        let mut index = self.hash1(key);
        let hash2_value = self.hash2(key);

        while let Some(entry) = &mut self.data[index] {
            if !entry.deleted && entry.key == *key {
                entry.deleted = true;

                self.size -= 1;
                break;
            }

            index = (index + hash2_value) % self.capacity;
        }
    }

    // clears the hash map
    // Complexity analysis:
    // Best: O(n)
    // Worst: O(n)
    // Average: O(n)
    pub fn clear(&mut self) {
        self.data = vec![None; self.capacity];
        self.size = 0;
    }

    // returns a clone of the hash map
    // Complexity analysis:
    // Best: O(n)
    // Worst: O(n)
    // Average: O(n)
    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            capacity: self.capacity,
            size: self.size,
        }
    }

    // converts the hash map to a string
    // Complexity analysis:
    // Best: O(n)
    // Worst: O(n)
    // Average: O(n)
    pub fn to_string(&self) -> String {
        let mut values = vec![];

        for (index, entry) in self.data.iter().enumerate() {
            match entry {
                Some(e) if !e.deleted => {
                    let entry = format!("Bucket {:2} => K: {:?}, V: {:?}", index, e.key, e.value);
                    values.push(entry);
                    values.push(String::from("\n"));
                }
                // Some(e) if e.deleted => println!("Bucket {:2} => [Deleted]", index),
                //_ => println!("Bucket {:2} => [Empty]", index),
                _ => {}
            }
        }

        values.pop();
        values.join("")
    }
}

// iterator implementation for the HashMap

impl<'a, K, V> HashMap<K, V> {
    // returns an iterator over the data in the hash map
    pub fn iter(&'a self) -> HashMapIter<'a, K, V> {
        HashMapIter {
            data: &self.data,
            index: 0,
        }
    }
}

impl<'a, K, V> IntoIterator for &'a HashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = HashMapIter<'a, K, V>;

    // returns an iterator over the hash map
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// iterator definition for the hash map

pub struct HashMapIter<'a, K, V> {
    data: &'a Vec<Option<Entry<K, V>>>,
    index: usize,
}

impl<'a, K, V> Iterator for HashMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    // returns the next key-value pair in the hash map
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.data.len() {
            let entry_option = &self.data[self.index];
            self.index += 1;

            if let Some(entry) = entry_option {
                if !entry.deleted {
                    return Some((&entry.key, &entry.value));
                }
            }
        }

        None
    }
}
