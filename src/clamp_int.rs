/// Clamp an integer to inclusive bounds, rejecting reversed bounds.
pub fn clamp_int(value: i64, lower: i64, upper: i64) -> Result<i64, &'static str> {
    if lower > upper {
        return Err("Lower bound must not exceed upper bound");
    }
    Ok(value.clamp(lower, upper))
}
