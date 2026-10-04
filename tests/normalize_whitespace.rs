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
