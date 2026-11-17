#[test]
fn test_binary_search_ints_empty() {
    assert_eq!(rust_utils::binary_search_ints(&[], 5), -1);
}
#[test]
fn test_binary_search_ints_single_element() {
    assert_eq!(rust_utils::binary_search_ints(&[5], 5), 0);
    assert_eq!(rust_utils::binary_search_ints(&[5], 3), -1);
}
#[test]
fn test_binary_search_ints_multiple_elements() {
    let data = [1, 2, 4, 4, 4, 7, 9];
    assert_eq!(rust_utils::binary_search_ints(&data, 4), 2);
    assert_eq!(rust_utils::binary_search_ints(&data, 1), 0);
    assert_eq!(rust_utils::binary_search_ints(&data, 9), 6);
    assert_eq!(rust_utils::binary_search_ints(&data, 5), -1);
}
