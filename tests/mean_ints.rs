use rust_utils::mean_ints;

#[test]
fn test_mean_ints_normal() {
    let result = mean_ints(&[1, 2, 3, 4, 5]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3.0);
}

#[test]
fn test_mean_ints_single() {
    let result = mean_ints(&[42]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42.0);
}

#[test]
fn test_mean_ints_negative() {
    let result = mean_ints(&[-10, 0, 10]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0.0);
}

#[test]
fn test_mean_ints_fractional() {
    let result = mean_ints(&[1, 2]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1.5);
}

#[test]
fn test_mean_ints_empty() {
    let result = mean_ints(&[]);
    assert!(result.is_err());
}
