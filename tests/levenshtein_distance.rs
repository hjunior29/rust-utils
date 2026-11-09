#[cfg(test)]
mod tests {
    use rust_utils::levenshtein_distance;
    #[test]
    fn test_levenshtein_distance_empty() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        assert_eq!(levenshtein_distance("", "abc"), 3);
    }
    #[test]
    fn test_levenshtein_distance_identical() {
        assert_eq!(levenshtein_distance("kitten", "kitten"), 0);
        assert_eq!(levenshtein_distance("unicode", "unicode"), 0);
    }
    #[test]
    fn test_levenshtein_distance_classic() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("sunday", "saturday"), 3);
    }
    #[test]
    fn test_levenshtein_distance_unicode() {
        assert_eq!(levenshtein_distance("cafe", "café"), 1);
        assert_eq!(levenshtein_distance("🦀rust🦀", "🦀fast🦀"), 2);
    }
}
