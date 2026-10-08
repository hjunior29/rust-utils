pub fn range_ints(start: i32, end: i32, step: i32) -> Result<Vec<i32>, &'static str> {
    if step <= 0 {
        return Err("Step must be positive");
    }

    let mut result = Vec::new();
    let mut current = start;

    while current < end {
        result.push(current);
        match current.checked_add(step) {
            Some(next) => current = next,
            None => break,
        }
    }

    Ok(result)
}
