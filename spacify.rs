fn spacify(s: &str) -> String {
    s.chars()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
