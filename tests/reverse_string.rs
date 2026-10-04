use rust_utils::reverse_string;

#[test]
fn reverses_empty_ascii_and_unicode_strings() {
    for (input, expected) in [("", ""), ("a", "a"), ("hello", "olleh"), ("a😀é", "é😀a")] {
        assert_eq!(reverse_string(input), expected);
    }
}
