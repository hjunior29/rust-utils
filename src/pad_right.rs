pub fn pad_right(input: &str, target_length: usize, pad_char: char) -> String {
    let current_length = input.chars().count();
    if current_length >= target_length {
        input.to_string()
    } else {
        let padding_needed = target_length - current_length;
        let padding: String = std::iter::repeat_n(pad_char, padding_needed).collect();
        format!("{}{}", input, padding)
    }
}
