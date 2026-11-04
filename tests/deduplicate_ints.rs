#[cfg(test)]
mod tests {
    use rust_utils::deduplicate_ints;
    #[test]
    fn test_deduplicate_ints_normal() {
        let input = vec![1, 2, 2, 3, 1, 4];
        let expected = vec![1, 2, 3, 4];
        assert_eq!(deduplicate_ints(&input), expected);
    }
    #[test]
    fn test_deduplicate_ints_empty() {
        let input: Vec<i32> = Vec::new();
        let expected: Vec<i32> = Vec::new();
        assert_eq!(deduplicate_ints(&input), expected);
    }
    #[test]
    fn test_deduplicate_ints_no_duplicates() {
        let input = vec![5, 6, 7];
        let expected = vec![5, 6, 7];
        assert_eq!(deduplicate_ints(&input), expected);
    }
    #[test]
    fn test_deduplicate_ints_all_duplicates() {
        let input = vec![9, 9, 9, 9];
        let expected = vec![9];
        assert_eq!(deduplicate_ints(&input), expected);
    }
}
