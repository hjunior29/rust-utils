use rust_utils::word_count;

#[test]
fn counts_words_with_unicode_whitespace() {
    for (input, expected) in [
        ("", 0),
        (" \t\n", 0),
        ("hello", 1),
        (" hello  world\nagain ", 3),
        ("one\u{2003}two", 2),
    ] {
        assert_eq!(word_count(input), expected);
    }
}
