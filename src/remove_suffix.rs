pub fn remove_suffix(s: &str, suffix: &str) -> String {
    if suffix.is_empty() {
        return s.to_string();
    }
    if let Some(stripped) = s.strip_suffix(suffix) {
        stripped.to_string()
    } else {
        s.to_string()
    }
}
