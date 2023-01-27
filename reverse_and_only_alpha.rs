fn reverse_letters(s: &str) -> String {
    s.chars()
        .rev()
        .filter(|&e| e >= 'a' && e <= 'z' || e >= 'A' && e <= 'Z')
        .collect()
}
