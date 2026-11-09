pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len_a = a_chars.len();
    let len_b = b_chars.len();
    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }
    let mut prev_row: Vec<usize> = (0..=len_b).collect();
    let mut curr_row: Vec<usize> = vec![0; len_b + 1];
    for i in 1..=len_a {
        curr_row[0] = i;
        for j in 1..=len_b {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            let deletion = prev_row[j] + 1;
            let insertion = curr_row[j - 1] + 1;
            let substitution = prev_row[j - 1] + cost;
            curr_row[j] = deletion.min(insertion).min(substitution);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    prev_row[len_b]
}
