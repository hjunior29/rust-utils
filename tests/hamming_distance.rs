use rust_utils::hamming_distance;

#[test]
fn test_hamming_distance_exact_match() {
    let result = hamming_distance("karolin", "karolin");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_hamming_distance_differing() {
    let result = hamming_distance("karolin", "kathrin");
    assert_eq!(result, Ok(3));
}

#[test]
fn test_hamming_distance_empty_strings() {
    let result = hamming_distance("", "");
    assert_eq!(result, Ok(0));
}

#[test]
fn test_hamming_distance_unicode_characters() {
    let result = hamming_distance("café", "cafs");
    assert_eq!(result, Ok(1));
}

#[test]
fn test_hamming_distance_unequal_lengths() {
    let result = hamming_distance("abc", "abcd");
    assert!(result.is_err());
}
