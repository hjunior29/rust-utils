pub fn rotate_ints(sequence: &[i32], offset: usize) -> Vec<i32> {
    if sequence.is_empty() {
        return Vec::new();
    }
    let len = sequence.len();
    let effective_offset = offset % len;
    if effective_offset == 0 {
        return sequence.to_vec();
    }
    let mut result = Vec::with_capacity(len);
    result.extend_from_slice(&sequence[effective_offset..]);
    result.extend_from_slice(&sequence[..effective_offset]);
    result
}
