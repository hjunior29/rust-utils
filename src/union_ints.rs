use std::collections::HashSet;
pub fn union_ints(first: &[i32], second: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut seen = HashSet::new();
    for &value in first.iter().chain(second.iter()) {
        if seen.insert(value) {
            result.push(value);
        }
    }
    result
}
