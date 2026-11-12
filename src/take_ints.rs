pub fn take_ints(items: &[i32], n: i32) -> Result<Vec<i32>, &'static str> {
    if n < 0 {
        return Err("Negative count is not allowed");
    }
    let count = n as usize;
    let result = items.iter().take(count).copied().collect();
    Ok(result)
}
