fn high(s: &str) -> &str {
    let v = s.split(" ").collect::<Vec<_>>();
    let f = |w: &str| w.chars().map(|e| e as u32 - 96).sum::<u32>();
    v.iter().rev().max_by(|a, b| f(a).cmp(&f(b))).unwrap()
}
