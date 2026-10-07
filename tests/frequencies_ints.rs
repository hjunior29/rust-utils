use rust_utils::frequencies_ints;

#[test]
fn test_frequencies_ints_normal() {
    let input = vec![1, 2, 2, 3, 3, 3];
    let result = frequencies_ints(&input);

    assert_eq!(result.len(), 3);
    assert_eq!(result.get(&1), Some(&1));
    assert_eq!(result.get(&2), Some(&2));
    assert_eq!(result.get(&3), Some(&3));
}

#[test]
fn test_frequencies_ints_empty() {
    let input: Vec<i32> = Vec::new();
    let result = frequencies_ints(&input);

    assert!(result.is_empty());
}

#[test]
fn test_frequencies_ints_boundaries() {
    let input = vec![i32::MAX, i32::MIN, i32::MAX, 0, 0, 0];
    let result = frequencies_ints(&input);

    assert_eq!(result.get(&i32::MAX), Some(&2));
    assert_eq!(result.get(&i32::MIN), Some(&1));
    assert_eq!(result.get(&0), Some(&3));
}

#[test]
fn test_frequencies_ints_single_element() {
    let input = vec![42];
    let result = frequencies_ints(&input);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get(&42), Some(&1));
}
