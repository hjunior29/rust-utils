#[test]
fn test_empty_string() {
    assert!(rust_utils::is_ascii(""));
}

#[test]
fn test_pure_ascii() {
    assert!(rust_utils::is_ascii("Hello, World!"));
}

#[test]
fn test_non_ascii() {
    assert!(!rust_utils::is_ascii("Héllo"));
}
