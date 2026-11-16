pub fn chunk_ints(input: &[i32], size: usize) -> Result<Vec<Vec<i32>>, &'static str> {
    if size == 0 {
        return Err("Size must be positive");
    }

    let mut result = Vec::new();
    let mut chunk = Vec::new();

    for &item in input {
        chunk.push(item);
        if chunk.len() == size {
            result.push(chunk);
            chunk = Vec::new();
        }
    }

    if !chunk.is_empty() {
        result.push(chunk);
    }

    Ok(result)
}
