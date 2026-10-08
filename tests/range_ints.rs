#[cfg(test)]
mod tests {
    use rust_utils::range_ints;

    #[test]
    fn test_range_normal_step() {
        let res = range_ints(1, 10, 2).unwrap();
        assert_eq!(res, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_range_empty() {
        let res = range_ints(5, 5, 1).unwrap();
        assert!(res.is_empty());

        let res_reversed = range_ints(10, 1, 2).unwrap();
        assert!(res_reversed.is_empty());
    }

    #[test]
    fn test_range_invalid_step() {
        let res_zero = range_ints(1, 10, 0);
        assert!(res_zero.is_err());

        let res_negative = range_ints(1, 10, -1);
        assert!(res_negative.is_err());
    }

    #[test]
    fn test_range_boundary() {
        let res = range_ints(0, 4, 3).unwrap();
        assert_eq!(res, vec![0, 3]);
    }
}
