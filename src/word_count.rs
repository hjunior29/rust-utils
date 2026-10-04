/// Count whitespace-separated words.
pub fn word_count(value: &str) -> usize {
    value.split_whitespace().count()
}
