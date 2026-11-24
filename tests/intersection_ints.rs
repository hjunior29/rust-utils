use rust_utils::intersection_ints;
#[test]
fn test_intersection_ints_basic() {
    let first = vec![1, 2, 3, 4];
    let second = vec![3, 4, 5, 6];
    assert_eq!(intersection_ints(&first, &second), vec![3, 4]);
}
#[test]
fn test_intersection_ints_empty() {
    let first: Vec<i32> = vec![];
    let second = vec![1, 2, 3];
    assert_eq!(intersection_ints(&first, &second), Vec::<i32>::new());
}
