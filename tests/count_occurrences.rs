use rust_utils::count_occurrences;
#[test]
fn test_count_occurrences_normal() {
    assert_eq!(count_occurrences("banana", "ana"), 1);
    assert_eq!(count_occurrences("abababab", "ab"), 4);
}
#[test]
fn test_count_occurrences_empty_needle() {
    assert_eq!(count_occurrences("hello", ""), 0);
    assert_eq!(count_occurrences("", ""), 0);
}
#[test]
fn test_count_occurrences_no_match() {
    assert_eq!(count_occurrences("hello", "xyz"), 0);
}
#[test]
fn test_count_occurrences_overlapping() {
    assert_eq!(count_occurrences("aaaa", "aa"), 2);
}
