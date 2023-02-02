fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut v = s
        .split_whitespace()
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
    v.sort_by(|a, b| a.chars().last().unwrap().cmp(&b.chars().last().unwrap()));
    v
}
