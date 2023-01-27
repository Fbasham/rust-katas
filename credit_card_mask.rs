fn maskify(s: &str) -> String {
    match s.len() {
        0..=3 => s.to_string(),
        _ => "#".repeat(s.len() - 4) + &s[s.len() - 4..],
    }
}
