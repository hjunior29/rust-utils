use rust_utils::chunk_ints;

#[test]
fn test_chunk_ints_normal() {
    let input = vec![1, 2, 3, 4, 5];
    let result = chunk_ints(&input, 2).unwrap();
    let expected = vec![vec![1, 2], vec![3, 4], vec![5]];
    assert_eq!(result, expected);
}

#[test]
fn test_chunk_ints_exact_multiple() {
    let input = vec![10, 20, 30, 40];
    let result = chunk_ints(&input, 2).unwrap();
    let expected = vec![vec![10, 20], vec![30, 40]];
    assert_eq!(result, expected);
}

#[test]
fn test_chunk_ints_empty_input() {
    let input: Vec<i32> = Vec::new();
    let result = chunk_ints(&input, 3).unwrap();
    let expected: Vec<Vec<i32>> = Vec::new();
    assert_eq!(result, expected);
}

#[test]
fn test_chunk_ints_size_larger_than_input() {
    let input = vec![1, 2];
    let result = chunk_ints(&input, 5).unwrap();
    let expected = vec![vec![1, 2]];
    assert_eq!(result, expected);
}

#[test]
fn test_chunk_ints_invalid_zero_size() {
    let input = vec![1, 2, 3];
    let result = chunk_ints(&input, 0);
    assert!(result.is_err());
}
