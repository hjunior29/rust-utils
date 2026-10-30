pub fn first_non_blank<'a, I>(sequence: I) -> Option<&'a str>
where
    I: IntoIterator<Item = &'a str>,
{
    #[allow(clippy::manual_find)]
    for item in sequence {
        if !item.chars().all(|c| c.is_whitespace()) {
            return Some(item);
        }
    }
    None
}
