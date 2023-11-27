#[allow(unused_imports)]
use crate::models::hash_map::HashMap;

#[test]
fn test_all() {
    let mut map = HashMap::new();
    assert_eq!(map.len(), 0);

    map.insert("a", 1);
    map.insert("b", 2);
    assert_eq!(map.len(), 2);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), None);

    map.remove(&"a");
    assert_eq!(map.get(&"a"), None);
    assert_eq!(map.len(), 1);

    map.insert("a", 3);
    assert_eq!(map.get(&"a"), Some(&3));
    assert_eq!(map.get(&"b"), Some(&2));

    map.clear();
    assert_eq!(map.len(), 0);

    let mut map3 = HashMap::new();

    for i in 0..1_000_000 {
        let key = format!("key{}", i);
        map3.insert(key, i);
    }

    assert_eq!(map3.len(), 1_000_000);

    for i in 0..1_000_000 {
        let key = format!("key{}", i);
        assert_eq!(map3.get(&key), Some(&i));
    }

    for i in 0..500_000 {
        let key = format!("key{}", i);
        map3.remove(&key);
    }

    assert_eq!(map3.len(), 500_000);

    for i in 0..500_000 {
        let key = format!("key{}", i);
        assert_eq!(map3.get(&key), None);
    }

    for i in 500_000..1_000_000 {
        let key = format!("key{}", i);
        assert_eq!(map3.get(&key), Some(&i));
    }

    for i in 750_000..1_000_000 {
        let key = format!("key{}", i);
        map3.remove(&key);
    }

    assert_eq!(map3.len(), 250_000);

    for i in 750_000..1_000_000 {
        let key = format!("key{}", i);
        assert_eq!(map3.get(&key), None);
    }

    for i in 500_000..750_000 {
        let key = format!("key{}", i);
        assert_eq!(map3.get(&key), Some(&i));
    }

    map3.clear();
    assert_eq!(map3.len(), 0);
}
