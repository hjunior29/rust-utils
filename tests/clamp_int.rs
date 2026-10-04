use rust_utils::clamp_int;

#[test]
fn clamps_interior_endpoints_and_equal_bounds() {
    for (value, lower, upper, expected) in [
        (5, 0, 10, 5),
        (-1, 0, 10, 0),
        (11, 0, 10, 10),
        (0, 0, 10, 0),
        (10, 0, 10, 10),
        (8, 3, 3, 3),
    ] {
        assert_eq!(clamp_int(value, lower, upper), Ok(expected));
    }
    assert!(clamp_int(5, 10, 0).is_err());
}
