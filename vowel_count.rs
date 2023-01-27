fn get_count(s: &str) -> usize {
    s.chars()
        .map(|e| if "aeiou".contains(e) { 1 } else { 0 })
        .sum()
}
