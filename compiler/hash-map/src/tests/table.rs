#[allow(unused_imports)]
use crate::models::table::Table;

#[test]
fn test_all() {
    let mut table = Table::new();
    assert_eq!(table.len(), 0);

    table.put("a");
    table.put("b");
    assert_eq!(table.len(), 2);

    assert_eq!(table.get(&"a"), Some(&1));
    assert_eq!(table.get(&"b"), Some(&2));
    assert_eq!(table.get(&"c"), None);

    table.remove(&"a");
    assert_eq!(table.get(&"a"), None);
    assert_eq!(table.len(), 1);

    table.insert("a", 3);
    assert_eq!(table.get(&"a"), Some(&3));
    assert_eq!(table.get(&"b"), Some(&2));

    table.clear();
    assert_eq!(table.len(), 0);

    let mut table3 = Table::new();

    for i in 0..1_000_000 {
        let key = format!("key{}", i);
        table3.put(key);
    }

    assert_eq!(table3.len(), 1_000_000);

    for i in 0..1_000_000 {
        let key = format!("key{}", i);
        let i = i + 1;
        assert_eq!(table3.get(&key), Some(&i));
    }

    for i in 0..500_000 {
        let key = format!("key{}", i);
        table3.remove(&key);
    }

    assert_eq!(table3.len(), 500_000);

    for i in 0..500_000 {
        let key = format!("key{}", i);
        assert_eq!(table3.get(&key), None);
    }

    for i in 500_000..1_000_000 {
        let key = format!("key{}", i);
        let i = i + 1;
        assert_eq!(table3.get(&key), Some(&i));
    }

    for i in 750_000..1_000_000 {
        let key = format!("key{}", i);
        table3.remove(&key);
    }

    assert_eq!(table3.len(), 250_000);

    for i in 750_000..1_000_000 {
        let key = format!("key{}", i);
        assert_eq!(table3.get(&key), None);
    }

    for i in 500_000..750_000 {
        let key = format!("key{}", i);
        let i = i + 1;
        assert_eq!(table3.get(&key), Some(&i));
    }

    table3.clear();
    assert_eq!(table3.len(), 0);
}
