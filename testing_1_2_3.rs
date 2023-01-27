fn number(a: &[&str]) -> Vec<String> {
    a.iter()
        .enumerate()
        .map(|(i, e)| format!("{}: {}", i + 1, e))
        .collect()
}
