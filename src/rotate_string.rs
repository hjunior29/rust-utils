pub fn rotate_string(s: &str, offset: usize) -> String {
    if s.is_empty() {
        return String::new();
    }
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let effective_offset = offset % len;
    if effective_offset == 0 {
        return s.to_string();
    }
    let mut result = String::with_capacity(s.len());
    for &c in chars.iter().skip(effective_offset) {
        result.push(c);
    }
    for &c in chars.iter().take(effective_offset) {
        result.push(c);
    }
    result
}
