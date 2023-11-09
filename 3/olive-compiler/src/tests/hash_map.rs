#[allow(unused_imports)]
use crate::models::hash_map::HashMap;

#[test]
fn test_all() {
    let mut map = HashMap::new();
    assert_eq!(map.size(), 0);

    map.insert("a", 1);
    map.insert("b", 2);
    assert_eq!(map.size(), 2);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), None);

    map.remove(&"a");
    assert_eq!(map.get(&"a"), None);
    assert_eq!(map.size(), 1);

    map.insert("a", 3);
    assert_eq!(map.get(&"a"), Some(&3));
    assert_eq!(map.get(&"b"), Some(&2));

    let mut map2 = map.clone();
    map2.remove(&"a");

    assert_eq!(map2.size(), 1);
    assert_eq!(map.size(), 2);

    assert!(map.contains(&"a"));
    assert!(map2.contains(&"b"));

    map.clear();
    assert_eq!(map.size(), 0);
}
