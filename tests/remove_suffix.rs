use rust_utils::remove_suffix;

#[test]
fn test_remove_suffix_normal() {
    let result = remove_suffix("filename.txt", ".txt");
    assert_eq!(result, "filename");
}

#[test]
fn test_remove_suffix_absent() {
    let result = remove_suffix("filename.log", ".txt");
    assert_eq!(result, "filename.log");
}

#[test]
fn test_remove_suffix_empty_suffix() {
    let result = remove_suffix("filename.txt", "");
    assert_eq!(result, "filename.txt");
}

#[test]
fn test_remove_suffix_empty_string() {
    let result = remove_suffix("", ".txt");
    assert_eq!(result, "");
}

#[test]
fn test_remove_suffix_exact_match() {
    let result = remove_suffix(".txt", ".txt");
    assert_eq!(result, "");
}
