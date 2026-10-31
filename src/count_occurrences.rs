pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut idx = 0;
    while let Some(pos) = haystack[idx..].find(needle) {
        count += 1;
        idx += pos + needle.len();
    }
    count
}
