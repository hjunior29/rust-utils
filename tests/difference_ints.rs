#[cfg(test)]
mod tests {
    use rust_utils::difference_ints;
    #[test]
    fn test_difference_ints_normal() {
        let first = vec![1, 2, 3, 2, 4];
        let second = vec![2, 5];
        let expected = vec![1, 3, 4];
        assert_eq!(difference_ints(&first, &second), expected);
    }
    #[test]
    fn test_difference_ints_empty() {
        let first: Vec<i32> = vec![];
        let second = vec![1, 2];
        assert_eq!(difference_ints(&first, &second), Vec::<i32>::new());
        let first = vec![1, 2];
        let second: Vec<i32> = vec![];
        let expected = vec![1, 2];
        assert_eq!(difference_ints(&first, &second), expected);
    }
}
