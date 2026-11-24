pub fn intersection_ints(first: &[i32], second: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &item in first {
        if second.contains(&item) && !result.contains(&item) {
            result.push(item);
        }
    }
    result
}
