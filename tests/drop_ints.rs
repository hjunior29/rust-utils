#[cfg(test)]
mod tests {
    use rust_utils::drop_ints;
    #[test]
    fn test_drop_ints_normal() {
        let input = vec![1, 2, 3, 4, 5];
        let result = drop_ints(&input, 2).unwrap();
        assert_eq!(result, vec![3, 4, 5]);
    }
    #[test]
    fn test_drop_ints_empty() {
        let input: Vec<i32> = vec![];
        let result = drop_ints(&input, 3).unwrap();
        assert_eq!(result, Vec::<i32>::new());
    }
    #[test]
    fn test_drop_ints_boundary_zero() {
        let input = vec![10, 20, 30];
        let result = drop_ints(&input, 0).unwrap();
        assert_eq!(result, vec![10, 20, 30]);
    }
    #[test]
    fn test_drop_ints_boundary_excess() {
        let input = vec![5, 6];
        let result = drop_ints(&input, 10).unwrap();
        assert_eq!(result, Vec::<i32>::new());
    }
    #[test]
    fn test_drop_ints_invalid_negative() {
        let input = vec![1, 2, 3];
        let result = drop_ints(&input, -1);
        assert!(result.is_err());
    }
}
