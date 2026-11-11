#[cfg(test)]
mod tests {
    use rust_utils::min_max_ints;
    #[test]
    fn test_min_max_ints_normal() {
        let result = min_max_ints(&[4, 1, 9, 3, 7]).unwrap();
        assert_eq!(result, (1, 9));
    }
    #[test]
    fn test_min_max_ints_single() {
        let result = min_max_ints(&[42]).unwrap();
        assert_eq!(result, (42, 42));
    }
    #[test]
    fn test_min_max_ints_negative() {
        let result = min_max_ints(&[-5, -1, -10, -3]).unwrap();
        assert_eq!(result, (-10, -1));
    }
    #[test]
    fn test_min_max_ints_empty() {
        let result = min_max_ints(&[]);
        assert!(result.is_err());
    }
}
