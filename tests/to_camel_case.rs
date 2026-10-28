use rust_utils::to_camel_case;
#[test]
fn test_to_camel_case_basic() {
    assert_eq!(to_camel_case("hello_world"), "helloWorld");
}
#[test]
fn test_to_camel_case_multiple_underscores() {
    assert_eq!(to_camel_case("foo__bar_baz"), "fooBarBaz");
}
#[test]
fn test_to_camel_case_leading_trailing() {
    assert_eq!(to_camel_case("_leading_trailing_"), "leadingTrailing");
}
#[test]
fn test_to_camel_case_empty() {
    assert_eq!(to_camel_case(""), "");
    assert_eq!(to_camel_case("___"), "");
}
#[test]
fn test_to_camel_case_single_word() {
    assert_eq!(to_camel_case("word"), "word");
    assert_eq!(to_camel_case("WORD"), "word");
}
