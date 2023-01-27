fn words_to_marks(s: &str) -> u32 {
    s.chars().map(|e| (e as u32) - 96).sum()
}
