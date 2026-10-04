/// Trim Unicode whitespace and collapse consecutive whitespace to one ASCII space.
pub fn normalize_whitespace(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_whitespace = true;
    for c in input.chars() {
        if c.is_whitespace() {
            if !in_whitespace {
                result.push(' ');
                in_whitespace = true;
            }
        } else {
            result.push(c);
            in_whitespace = false;
        }
    }
    if result.ends_with(' ') {
        result.pop();
    }
    result
}
