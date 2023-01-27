fn add_length(s: &str) -> Vec<String> {
    return s
        .split(" ")
        .map(|e| e.to_string() + " " + &e.len().to_string())
        .collect();
}
