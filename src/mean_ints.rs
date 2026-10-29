pub fn mean_ints(values: &[i32]) -> Result<f64, &'static str> {
    if values.is_empty() {
        return Err("cannot compute the mean of an empty slice");
    }

    let mut sum: i64 = 0;
    for &value in values {
        sum += value as i64;
    }

    let mean = sum as f64 / values.len() as f64;
    Ok(mean)
}
