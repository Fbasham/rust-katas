fn triple_x(s: &str) -> bool {
    let i = s.chars().position(|e| e == 'x').unwrap_or(0);
    &s[i..(i + 3).min(s.len())] == "xxx"
}
