fn remove(s: &str) -> String {
    s.split(" ")
        .filter(|&e| e.chars().filter(|&c| c == '!').count() != 1)
        .collect::<Vec<_>>()
        .join(" ")
}
