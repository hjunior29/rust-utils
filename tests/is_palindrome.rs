#[test]
fn test_empty_string() {
    assert!(rust_utils::is_palindrome(""));
}

#[test]
fn test_single_character() {
    assert!(rust_utils::is_palindrome("a"));
}

#[test]
fn test_simple_palindrome() {
    assert!(rust_utils::is_palindrome("racecar"));
}

#[test]
fn test_not_palindrome() {
    assert!(!rust_utils::is_palindrome("hello"));
}

#[test]
fn test_case_sensitive() {
    assert!(!rust_utils::is_palindrome("Racecar"));
}

#[test]
fn test_unicode_palindrome() {
    assert!(rust_utils::is_palindrome("level"));
}
