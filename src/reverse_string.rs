/// Reverse a string by Unicode scalar values.
pub fn reverse_string(value: &str) -> String {
    value.chars().rev().collect()
}
