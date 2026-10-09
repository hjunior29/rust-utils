pub fn flatten_ints<I, J>(sequences: I) -> Vec<i32>
where
    I: IntoIterator<Item = J>,
    J: IntoIterator<Item = i32>,
{
    let mut result = Vec::new();
    for seq in sequences {
        for item in seq {
            result.push(item);
        }
    }
    result
}
