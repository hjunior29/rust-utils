pub fn min_max_ints(values: &[i32]) -> Result<(i32, i32), &'static str> {
    if values.is_empty() {
        return Err("Empty sequence provided");
    }
    let mut min_val = values[0];
    let mut max_val = values[0];
    for &val in values.iter().skip(1) {
        if val < min_val {
            min_val = val;
        }
        if val > max_val {
            max_val = val;
        }
    }
    Ok((min_val, max_val))
}
