pub fn pad_left(input: &str, target_length: usize, pad_char: char) -> String {
    let current_length = input.chars().count();
    if current_length >= target_length {
        return input.to_string();
    }
    let padding_count = target_length - current_length;
    let mut result = String::with_capacity(input.len() + padding_count * pad_char.len_utf8());
    for _ in 0..padding_count {
        result.push(pad_char);
    }
    result.push_str(input);
    result
}
