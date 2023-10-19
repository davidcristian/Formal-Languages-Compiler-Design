mod models;
use models::table::Table;

fn test_table() {
    let mut table = Table::new();
    assert_eq!(table.size(), 0);

    table.insert("a");
    table.insert("b");
    assert_eq!(table.size(), 2);

    assert_eq!(table.get(&"a"), Some(&0));
    assert_eq!(table.get(&"b"), Some(&1));
    assert_eq!(table.get(&"c"), None);

    table.remove(&"a");
    assert_eq!(table.get(&"a"), None);
    assert_eq!(table.size(), 1);

    table.put("a", 1);
    assert_eq!(table.get(&"a"), Some(&1));
    assert_eq!(table.get(&"b"), Some(&1));

    table.clear();
    assert_eq!(table.size(), 0);
}

fn main() {
    test_table();
    println!("All tests passed.");
}
