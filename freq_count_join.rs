fn freq_seq(s: &str, sep: &str) -> String {
    s.chars()
        .map(|e| s.chars().filter(|&x| e == x).count().to_string())
        .collect::<Vec<_>>()
        .join(sep)
}
