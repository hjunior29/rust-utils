pub fn sum_digits(n: i64) -> Result<u32, &'static str> {
    if n < 0 {
        return Err("negative input is not allowed");
    }
    let mut total: u32 = 0;
    let mut current = n;
    while current > 0 {
        let digit = current % 10;
        total += digit as u32;
        current /= 10;
    }
    Ok(total)
}
