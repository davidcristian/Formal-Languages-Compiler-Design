pub fn vector<T>(capacity: usize) -> Vec<Option<T>> {
    let mut data = Vec::with_capacity(capacity);
    data.extend((0..capacity).map(|_| None));
    data
}
