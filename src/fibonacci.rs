pub fn fibonacci(n: i32) -> Result<u64, &'static str> {
    if !(0..=30).contains(&n) {
        return Err("Input must be between 0 and 30 inclusive");
    }
    if n == 0 {
        return Ok(0);
    }
    if n == 1 {
        return Ok(1);
    }
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    Ok(curr)
}
