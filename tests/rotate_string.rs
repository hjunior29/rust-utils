#[cfg(test)]
mod tests {
    use rust_utils::rotate_string;
    #[test]
    fn test_rotate_basic() {
        assert_eq!(rotate_string("hello", 2), "llohe");
    }
    #[test]
    fn test_rotate_empty() {
        assert_eq!(rotate_string("", 3), "");
    }
    #[test]
    fn test_rotate_overflow_offset() {
        assert_eq!(rotate_string("abc", 5), "cab");
    }
    #[test]
    fn test_rotate_zero_offset() {
        assert_eq!(rotate_string("test", 0), "test");
    }
    #[test]
    fn test_rotate_unicode() {
        assert_eq!(rotate_string("🦀rust", 1), "rust🦀");
    }
}
