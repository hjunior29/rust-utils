#[cfg(test)]
mod tests {
    use rust_utils::first_non_blank;
    #[test]
    fn test_first_non_blank_normal() {
        let items = vec!["", "   ", "hello", "world"];
        assert_eq!(first_non_blank(items), Some("hello"));
    }
    #[test]
    fn test_first_non_blank_empty_and_blank() {
        let items = vec!["", "   ", "\t\n"];
        assert_eq!(first_non_blank(items), None);
    }
    #[test]
    fn test_first_non_blank_boundary_single() {
        let items = vec!["single"];
        assert_eq!(first_non_blank(items), Some("single"));
    }
}
