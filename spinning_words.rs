fn spin_words(s: &str) -> String {
    s.split_whitespace()
        .map(|e| match e.len() < 5 {
            true => e.to_string(),
            false => e.chars().rev().collect(),
        })
        .collect::<Vec<String>>()
        .join(" ")
        .to_string()
}
