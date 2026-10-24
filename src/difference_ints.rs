pub fn difference_ints(first: &[i32], second: &[i32]) -> Vec<i32> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    let exclude: HashSet<i32> = second.iter().copied().collect();
    for &val in first {
        if !exclude.contains(&val) && seen.insert(val) {
            result.push(val);
        }
    }
    result
}
