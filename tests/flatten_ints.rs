#[cfg(test)]
mod tests {
    use rust_utils::flatten_ints;
    #[test]
    fn test_flatten_ints_normal() {
        let input = vec![vec![1, 2], vec![3], vec![], vec![4, 5]];
        let expected = vec![1, 2, 3, 4, 5];
        assert_eq!(flatten_ints(input), expected);
    }
    #[test]
    fn test_flatten_ints_empty() {
        let input: Vec<Vec<i32>> = vec![];
        let expected: Vec<i32> = vec![];
        assert_eq!(flatten_ints(input), expected);
    }
    #[test]
    fn test_flatten_ints_all_empty_subsequences() {
        let input = vec![vec![], vec![], vec![]];
        let expected: Vec<i32> = vec![];
        assert_eq!(flatten_ints(input), expected);
    }
}
