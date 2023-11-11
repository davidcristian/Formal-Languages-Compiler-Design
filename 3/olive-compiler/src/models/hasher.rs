use std::hash::Hasher;

pub struct Djb2Hasher {
    hash: u64,
}

impl Djb2Hasher {
    pub fn new() -> Djb2Hasher {
        Djb2Hasher { hash: 5381 }
    }
}

impl Hasher for Djb2Hasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.hash = (self.hash << 5)
                .wrapping_add(self.hash)
                .wrapping_add(*byte as u64);
        }
    }
}
