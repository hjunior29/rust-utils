/// Return the longest shared prefix by Unicode code points.
pub fn common_prefix(a: &str, b: &str) -> String {
    let mut out = String::new();
    for (c1, c2) in a.chars().zip(b.chars()) {
        if c1 == c2 {
            out.push(c1);
        } else {
            break;
        }
    }
    out
}
