#[test]
fn test_slugify_basic() {
    assert_eq!(rust_utils::slugify("Hello, World!"), "hello-world");
}

#[test]
fn test_slugify_empty() {
    assert_eq!(rust_utils::slugify(""), "");
}

#[test]
fn test_slugify_whitespace_and_symbols() {
    assert_eq!(
        rust_utils::slugify("   Multiple    Spaces & Symbols! "),
        "multiple-spaces-symbols"
    );
}

#[test]
fn test_slugify_already_slugged() {
    assert_eq!(
        rust_utils::slugify("rust-edition-2021"),
        "rust-edition-2021"
    );
}
