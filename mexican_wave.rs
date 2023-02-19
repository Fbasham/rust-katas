fn wave(s: &str) -> Vec<String> {
    (0..s.len())
        .filter(|&i| &s[i..i + 1] != " ")
        .map(|i| format!("{}{}{}", &s[..i], &s[i..i + 1].to_uppercase(), &s[i + 1..]))
        .collect()
}
