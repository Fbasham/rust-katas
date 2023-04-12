fn rotate(s: &str) -> Vec<String> {
    (1..s.len()+1).map(|i| s[i..].to_string()+&s[..i]).collect()
}