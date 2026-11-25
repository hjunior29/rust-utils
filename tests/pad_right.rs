use rust_utils::pad_right;
#[test]
fn test_pad_right_normal() {
    assert_eq!(pad_right("abc", 5, 'x'), "abcxx");
}
#[test]
fn test_pad_right_already_long_enough() {
    assert_eq!(pad_right("abcdef", 4, 'x'), "abcdef");
}
#[test]
fn test_pad_right_empty_input() {
    assert_eq!(pad_right("", 3, '0'), "000");
}
#[test]
fn test_pad_right_unicode() {
    assert_eq!(pad_right("rust", 6, '🦀'), "rust🦀🦀");
}
