use rust_utils::union_ints;
#[test]
fn test_union_ints_basic() {
    let first = vec![1, 2, 3];
    let second = vec![3, 4, 5];
    let expected = vec![1, 2, 3, 4, 5];
    assert_eq!(union_ints(&first, &second), expected);
}
#[test]
fn test_union_ints_empty() {
    let first: Vec<i32> = vec![];
    let second: Vec<i32> = vec![];
    let expected: Vec<i32> = vec![];
    assert_eq!(union_ints(&first, &second), expected);
}
#[test]
fn test_union_ints_duplicates() {
    let first = vec![1, 2, 2, 1];
    let second = vec![2, 3, 3, 4];
    let expected = vec![1, 2, 3, 4];
    assert_eq!(union_ints(&first, &second), expected);
}
