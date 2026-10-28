pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_uppercase() {
            let prev_lower = if i > 0 {
                chars[i - 1].is_ascii_lowercase()
            } else {
                false
            };
            let next_lower = if i + 1 < len {
                chars[i + 1].is_ascii_lowercase()
            } else {
                false
            };
            let should_insert_underscore = prev_lower
                || (next_lower && i > 0 && chars[i - 1].is_ascii_uppercase())
                || (i > 0
                    && !chars[i - 1].is_ascii_uppercase()
                    && !chars[i - 1].is_ascii_lowercase());
            if should_insert_underscore {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else if c == '-' || c == ' ' {
            result.push('_');
        } else {
            result.push(c);
        }
    }
    result
}
