use super::hasher::Djb2Hasher;
use std::hash::Hasher;

const INITIAL_CAPACITY: usize = 16;
const RESIZE_FACTOR: usize = 2;
const LOAD_FACTOR: f64 = 0.75;

#[derive(Clone)]
struct Entry<K, V> {
    key: K,
    value: V,
    probe_count: usize,
}

pub struct HashMap<K, V> {
    data: Vec<Option<Entry<K, V>>>,
    capacity: usize,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    // creates a new empty hash map
    pub fn new() -> Self {
        Self {
            data: vec![None; INITIAL_CAPACITY],
            capacity: INITIAL_CAPACITY,
            size: 0,
        }
    }

    // hash using the djb2 algorithm
    fn hash(&self, key: &K) -> usize {
        let mut hasher = Djb2Hasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    // resizes the hash map to increase its current capacity by RESIZE_FACTOR
    // Complexity analysis:
    // Best: O(n)
    // Worst: O(n)
    // Average: O(n)
    fn resize(&mut self) {
        self.size = 0;
        self.capacity *= RESIZE_FACTOR;

        let old_data = std::mem::replace(&mut self.data, vec![None; self.capacity]);
        for entry in old_data.into_iter().flatten() {
            self.insert(entry.key, entry.value);
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

        let mut index = self.hash(&key);
        let mut entry = Entry {
            key,
            value,
            probe_count: 0,
        };

        loop {
            match &mut self.data[index] {
                Some(existing_entry) => {
                    // if the probe count of the existing entry is less than that
                    // of the entry to be inserted, swap the entries
                    if existing_entry.probe_count < entry.probe_count {
                        std::mem::swap(existing_entry, &mut entry);
                    }

                    // if the key matches, update the value and do not increment the size
                    if existing_entry.key == entry.key {
                        existing_entry.value = entry.value;
                        return;
                    }
                }
                None => {
                    // Found an empty slot, place the entry here
                    self.data[index] = Some(entry);
                    self.size += 1;

                    return;
                }
            }

            // keep looking for an empty slot
            entry.probe_count += 1;
            index = (index + 1) % self.capacity;
        }
    }

    // returns the value for the given key
    // Complexity analysis:
    // Best: O(1)
    // Worst: O(n)
    // Average: O(1)
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut index = self.hash(key);
        let mut probe_count = 0;

        // keep looking for the entry until we find an empty slot or the probe count exceeds
        while let Some(entry) = &self.data[index] {
            // stop the search if the probe count exceeds that of the entry's probe count
            if probe_count > entry.probe_count {
                break;
            }

            // return the value if the key matches
            if entry.key == *key {
                return Some(&entry.value);
            }

            // keep looking
            index = (index + 1) % self.capacity;
            probe_count += 1;
        }

        None
    }

    // returns true if the hash map contains the given key
    // Complexity analysis:
    // Best: O(1)
    // Worst: O(n)
    // Average: O(1)
    pub fn contains(&self, key: &K) -> bool {
        let mut index = self.hash(key);
        let mut probe_count = 0;

        // keep looking for the entry until we find an empty slot or the probe count exceeds
        while let Some(entry) = &self.data[index] {
            // stop the search if the probe count exceeds that of the entry's probe count
            if probe_count > entry.probe_count {
                return false;
            }

            // return true if the key matches
            if entry.key == *key {
                return true;
            }

            // keep looking
            index = (index + 1) % self.capacity;
            probe_count += 1;
        }

        false
    }

    // removes the key-value pair for the given key
    // Complexity analysis:
    // Best: O(1)
    // Worst: O(n)
    // Average: O(1)
    pub fn remove(&mut self, key: &K) {
        let mut index = self.hash(key);
        while let Some(entry) = &self.data[index] {
            if entry.key == *key {
                // remove the entry
                self.data[index] = None;
                self.size -= 1;

                // reinsert subsequent entries
                let mut next_index = (index + 1) % self.capacity;
                while let Some(next_entry) = self.data[next_index].take() {
                    self.insert(next_entry.key, next_entry.value);
                    self.size -= 1; // the size was incremented by the insert method

                    next_index = (next_index + 1) % self.capacity;
                }

                return;
            }

            index = (index + 1) % self.capacity;
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
            if let Some(entry) = &self.data[self.index] {
                self.index += 1;
                return Some((&entry.key, &entry.value));
            } else {
                self.index += 1;
            }
        }

        None
    }
}
