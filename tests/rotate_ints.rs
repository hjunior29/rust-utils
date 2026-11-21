use rust_utils::rotate_ints;
#[test]
fn test_rotate_ints_normal() {
    let input = vec![1, 2, 3, 4, 5];
    let result = rotate_ints(&input, 2);
    assert_eq!(result, vec![3, 4, 5, 1, 2]);
    assert_eq!(input, vec![1, 2, 3, 4, 5]);
}
#[test]
fn test_rotate_ints_empty() {
    let input: Vec<i32> = Vec::new();
    let result = rotate_ints(&input, 3);
    assert_eq!(result, Vec::<i32>::new());
}
#[test]
fn test_rotate_ints_boundary_zero() {
    let input = vec![10, 20, 30];
    let result = rotate_ints(&input, 0);
    assert_eq!(result, vec![10, 20, 30]);
}
#[test]
fn test_rotate_ints_large_offset() {
    let input = vec![1, 2, 3];
    let result = rotate_ints(&input, 5);
    assert_eq!(result, vec![3, 1, 2]);
}
