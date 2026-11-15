use rust_utils::truncate_string;

#[test]
fn test_truncate_string_normal() {
    assert_eq!(truncate_string("hello", 3), "hel");
}

#[test]
fn test_truncate_string_empty() {
    assert_eq!(truncate_string("", 5), "");
}

#[test]
fn test_truncate_string_boundary() {
    assert_eq!(truncate_string("abc", 3), "abc");
    assert_eq!(truncate_string("abc", 5), "abc");
    assert_eq!(truncate_string("abc", 0), "");
}

#[test]
fn test_truncate_string_unicode() {
    assert_eq!(truncate_string("🦀🔥🎉", 2), "🦀🔥");
}
