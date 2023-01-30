fn balanced_num(n: u64) -> String {
    let s = n.to_string();
    let f = |s: &str| s.chars().map(|e| (e as u32) - 48).sum::<u32>();
    let a = &s[..s.len() / 2 - (if s.len() % 2 == 1 { 0 } else { 1 })];
    let b = &s[(s.len() / 2 + 1)..];
    match f(a) == f(b) {
        true => "Balanced".to_string(),
        false => "Not Balanced".to_string(),
    }
}
