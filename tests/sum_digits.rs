use rust_utils::sum_digits;

#[test]
fn test_sum_digits_normal() {
    assert_eq!(sum_digits(12345).unwrap(), 15);
    assert_eq!(sum_digits(987).unwrap(), 24);
}

#[test]
fn test_sum_digits_boundary() {
    assert_eq!(sum_digits(0).unwrap(), 0);
    assert_eq!(sum_digits(7).unwrap(), 7);
}

#[test]
fn test_sum_digits_invalid() {
    assert!(sum_digits(-1).is_err());
    assert!(sum_digits(-123).is_err());
}
