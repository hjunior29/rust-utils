pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    if a.chars().count() != b.chars().count() {
        return Err("Strings must have equal length");
    }

    let distance = a
        .chars()
        .zip(b.chars())
        .filter(|(char_a, char_b)| char_a != char_b)
        .count();

    Ok(distance)
}
