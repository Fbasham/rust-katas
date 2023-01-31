fn longest_consec(v: Vec<&str>, k: usize) -> String {
    if k > v.len() {
        return "".to_string();
    }
    (0..=v.len().saturating_sub(k))
        .map(|i| v[i..i + k].join(""))
        .rev()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap_or("".to_string())
}
