fn valid_spacing(s: &str) -> bool {
    s.trim().len() == s.len() && !s.contains("  ")
}
