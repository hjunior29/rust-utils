use std::collections::HashMap;

/// Count the occurrences of each integer in a sequence.
///
/// # Arguments
///
/// * `sequence` - A slice of integers to analyze.
///
/// # Returns
///
/// A `HashMap` mapping each integer to its frequency count.
pub fn frequencies_ints(sequence: &[i32]) -> HashMap<i32, usize> {
    let mut counts = HashMap::new();

    for &number in sequence {
        *counts.entry(number).or_insert(0) += 1;
    }

    counts
}
