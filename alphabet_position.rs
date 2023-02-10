fn alphabet_position(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|&c| c.is_alphabetic())
        .map(|e| (e as u8 - 96).to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
