use rust_utils::fibonacci;

#[test]
fn test_fibonacci_base_cases() {
    assert_eq!(fibonacci(0), Ok(0));
    assert_eq!(fibonacci(1), Ok(1));
}

#[test]
fn test_fibonacci_small_values() {
    assert_eq!(fibonacci(2), Ok(1));
    assert_eq!(fibonacci(3), Ok(2));
    assert_eq!(fibonacci(4), Ok(3));
    assert_eq!(fibonacci(5), Ok(5));
    assert_eq!(fibonacci(6), Ok(8));
}

#[test]
fn test_fibonacci_boundary_upper() {
    assert_eq!(fibonacci(30), Ok(832040));
}

#[test]
fn test_fibonacci_invalid_negative() {
    assert!(fibonacci(-1).is_err());
    assert!(fibonacci(-10).is_err());
}

#[test]
fn test_fibonacci_invalid_too_large() {
    assert!(fibonacci(31).is_err());
    assert!(fibonacci(100).is_err());
}
