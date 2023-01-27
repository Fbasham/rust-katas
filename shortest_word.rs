fn find_short(s: &str) -> u32 {
    s.split_whitespace().map(|e| e.len() as u32).min().unwrap()
}
