fn solution(s: &str, k: usize) -> String {
    match k >= s.len() {
        true => s.to_string(),
        false => format!("{}...", &s[..k]).to_string(),
    }
}
