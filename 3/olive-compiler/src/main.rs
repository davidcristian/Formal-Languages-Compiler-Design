mod models;
use models::table::Table;

fn test_table() {
    let mut table = Table::new();
    assert_eq!(table.size(), 0);

    table.insert("a");
    table.insert("b");
    assert_eq!(table.size(), 2);

    assert_eq!(table.get(&"a"), Some(&1));
    assert_eq!(table.get(&"b"), Some(&2));
    assert_eq!(table.get(&"c"), None);

    table.remove(&"a");
    assert_eq!(table.get(&"a"), None);
    assert_eq!(table.size(), 1);

    table.put("a", 3);
    assert_eq!(table.get(&"a"), Some(&3));
    assert_eq!(table.get(&"b"), Some(&2));

    let mut table2 = table.clone();
    table2.remove(&"a");

    assert_eq!(table2.size(), 1);
    assert_eq!(table.size(), 2);

    assert!(table.contains(&"a"));
    assert!(table2.contains(&"b"));

    table.clear();
    assert_eq!(table.size(), 0);
}

fn main() {
    test_table();
    println!("All tests passed.");
}
