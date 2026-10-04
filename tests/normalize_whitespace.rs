#[cfg(test)]
mod tests {
    use rust_utils::normalize_whitespace;
    #[test]
    fn test_normal_case() {
        assert_eq!(normalize_whitespace("hello   world"), "hello world");
    }
    #[test]
    fn test_empty_string() {
        assert_eq!(normalize_whitespace(""), "");
    }
    #[test]
    fn test_whitespace_only() {
        assert_eq!(normalize_whitespace("   "), "");
    }
    #[test]
    fn test_leading_and_trailing() {
        assert_eq!(normalize_whitespace("   foo bar   "), "foo bar");
    }
    #[test]
    fn test_newlines_and_tabs() {
        assert_eq!(normalize_whitespace("line1\n\tline2"), "line1 line2");
    }
}

#[test]
fn preserves_unicode_text_and_normalizes_unicode_whitespace() {
    assert_eq!(
        rust_utils::normalize_whitespace("\u{2003}🦀\u{a0}\u{2009}go\u{202f}"),
        "🦀 go"
    );
    assert_eq!(rust_utils::normalize_whitespace("café"), "café");
}
