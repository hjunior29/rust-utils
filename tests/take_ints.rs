use rust_utils::take_ints;

#[test]
fn test_take_ints_normal() {
    let input = vec![10, 20, 30, 40, 50];
    let result = take_ints(&input, 3).unwrap();
    assert_eq!(result, vec![10, 20, 30]);
}

#[test]
fn test_take_ints_empty_input() {
    let input: Vec<i32> = vec![];
    let result = take_ints(&input, 2).unwrap();
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_take_ints_zero_count() {
    let input = vec![1, 2, 3];
    let result = take_ints(&input, 0).unwrap();
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_take_ints_exceed_length() {
    let input = vec![5, 6];
    let result = take_ints(&input, 10).unwrap();
    assert_eq!(result, vec![5, 6]);
}

#[test]
fn test_take_ints_negative_reject() {
    let input = vec![1, 2, 3];
    let result = take_ints(&input, -1);
    assert!(result.is_err());
}
