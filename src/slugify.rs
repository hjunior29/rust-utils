pub fn slugify(input: &str) -> String {
    let mut result = String::new();
    let mut last_was_hyphen = true;
    for c in input.chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            last_was_hyphen = false;
        } else if !last_was_hyphen {
            result.push('-');
            last_was_hyphen = true;
        }
    }
    if result.ends_with('-') {
        result.pop();
    }
    result
}
