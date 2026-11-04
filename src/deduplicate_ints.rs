pub fn deduplicate_ints(input: &[i32]) -> Vec<i32> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for &val in input {
        if seen.insert(val) {
            result.push(val);
        }
    }
    result
}
