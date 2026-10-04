#[cfg(test)]
mod tests {
    use rust_utils::common_prefix;
    #[test]
    fn test_common_prefix_normal() {
        assert_eq!(common_prefix("interspecies", "interstellar"), "inters");
        assert_eq!(common_prefix("apple", "apricot"), "ap");
    }
    #[test]
    fn test_common_prefix_empty() {
        assert_eq!(common_prefix("", "abc"), "");
        assert_eq!(common_prefix("abc", ""), "");
        assert_eq!(common_prefix("", ""), "");
    }
    #[test]
    fn test_common_prefix_identical() {
        assert_eq!(common_prefix("hello", "hello"), "hello");
    }
    #[test]
    fn test_common_prefix_no_match() {
        assert_eq!(common_prefix("dog", "cat"), "");
    }
    #[test]
    fn test_common_prefix_unicode() {
        assert_eq!(common_prefix("🦀crustacean", "🦀crab"), "🦀cr");
        assert_eq!(common_prefix("café", "cafeteria"), "caf");
    }
}
