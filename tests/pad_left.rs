use rust_utils::pad_left;

#[test]
fn test_pad_left_normal() {
    let result = pad_left("42", 5, '0');
    assert_eq!(result, "00042");
}

#[test]
fn test_pad_left_already_long_enough() {
    let result = pad_left("hello", 3, 'x');
    assert_eq!(result, "hello");
}

#[test]
fn test_pad_left_exact_length() {
    let result = pad_left("abc", 3, ' ');
    assert_eq!(result, "abc");
}

#[test]
fn test_pad_left_empty_input() {
    let result = pad_left("", 4, '-');
    assert_eq!(result, "----");
}

#[test]
fn test_pad_left_unicode_characters() {
    let result = pad_left("ü", 3, '★');
    assert_eq!(result, "★★ü");
}
