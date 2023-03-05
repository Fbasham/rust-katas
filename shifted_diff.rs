fn shifted_diff(s: &str, t: &str) -> Option<usize> {
    (0..s.len()).position(|i| s[s.len() - i..].to_string() + &s[0..s.len() - i] == t)
}
