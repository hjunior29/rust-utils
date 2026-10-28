pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut first_segment = true;
    for part in input.split('_') {
        if part.is_empty() {
            continue;
        }
        if first_segment {
            result.push_str(&part.to_lowercase());
            first_segment = false;
        } else {
            let mut chars = part.chars();
            if let Some(first_char) = chars.next() {
                result.push(first_char.to_ascii_uppercase());
                for c in chars {
                    result.push(c.to_ascii_lowercase());
                }
            }
        }
    }
    result
}
