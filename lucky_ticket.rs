fn luck_check(s: &str) -> Option<bool> {
    let n = s.len() / 2;
    let x = &s[..n].chars().map(|e| e as i32 - 48).sum::<i32>();
    let y = &s[n + (if s.len() % 2 == 1 { 1 } else { 0 })..]
        .chars()
        .map(|e| e as i32 - 48)
        .sum::<i32>();
    match s.is_empty() || s.chars().any(|c| !c.is_numeric()) {
        true => None,
        _ => Some(x == y),
    }
}
