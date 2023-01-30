fn order_weight(s: &str) -> String {
    let f = |s: String| s.chars().map(|e| (e as u32) - 48).sum::<u32>();
    let mut v = s
        .split_whitespace()
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
    v.sort_by(|a, b| f(a.to_string()).cmp(&f(b.to_string())).then(a.cmp(b)));
    v.join(" ")
}
