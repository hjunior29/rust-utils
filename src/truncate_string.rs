pub fn truncate_string(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect()
}
