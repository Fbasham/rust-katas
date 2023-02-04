fn solve(a: &str, b: &str) -> String {
    a.chars()
        .chain(b.chars())
        .filter(|&e| a.contains(e) && !b.contains(e) || !a.contains(e) && b.contains(e))
        .collect()
}
