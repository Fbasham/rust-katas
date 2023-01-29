fn vaporcode(s: &str) -> String {
    s.to_uppercase()
        .replace(" ", "")
        .chars()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("  ")
}
