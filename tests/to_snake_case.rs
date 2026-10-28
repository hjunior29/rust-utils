use rust_utils::to_snake_case;
#[test]
fn test_to_snake_case_basic() {
    assert_eq!(to_snake_case("CamelCase"), "camel_case");
    assert_eq!(to_snake_case("pascalCase"), "pascal_case");
    assert_eq!(to_snake_case("already_snake"), "already_snake");
}
#[test]
fn test_to_snake_case_empty() {
    assert_eq!(to_snake_case(""), "");
}
#[test]
fn test_to_snake_case_acronyms() {
    assert_eq!(to_snake_case("XMLParser"), "xml_parser");
    assert_eq!(to_snake_case("getHTTPResponse"), "get_http_response");
}
#[test]
fn test_to_snake_case_boundaries() {
    assert_eq!(to_snake_case("A"), "a");
    assert_eq!(to_snake_case("ABC"), "abc");
}
