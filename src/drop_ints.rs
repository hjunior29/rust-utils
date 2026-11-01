pub fn drop_ints(slice: &[i32], n: i32) -> Result<Vec<i32>, &'static str> {
    if n < 0 {
        return Err("Negative count is not allowed");
    }
    let mut count = 0;
    let mut result = Vec::new();
    let limit = n as usize;
    for &item in slice {
        if count >= limit {
            result.push(item);
        } else {
            count += 1;
        }
    }
    Ok(result)
}
