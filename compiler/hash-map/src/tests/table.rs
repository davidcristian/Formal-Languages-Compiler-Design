#[allow(unused_imports)]
use crate::models::table::Table;

#[test]
fn test_all() {
    let mut table = Table::new();
    assert_eq!(table.len(), 0);

    assert_eq!(table.put("a"), 1);
    assert_eq!(table.put("b"), 2);

    assert_eq!(table.put("a"), 1);
    assert_eq!(table.put("b"), 2);

    assert_eq!(table.len(), 2);

    assert_eq!(table.get(&1), Some(&"a"));
    assert_eq!(table.get(&2), Some(&"b"));
    assert_eq!(table.get(&3), None);

    table.clear();
    assert_eq!(table.len(), 0);

    assert_eq!(table.get(&1), None);
    assert_eq!(table.get(&2), None);
    assert_eq!(table.get(&3), None);
}
