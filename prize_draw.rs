fn rank(s: &str, w: Vec<i32>, n: usize) -> &str {
    if s.is_empty() {
        return "No participants";
    }
    let mut v: Vec<(i32, &str)> = s
        .split(",")
        .enumerate()
        .map(|(i, e)| {
            (
                (e.len() as i32 + e.to_lowercase().chars().map(|x| x as i32 - 96).sum::<i32>())
                    * w[i],
                e,
            )
        })
        .collect::<Vec<_>>();
    v.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    if n > v.len() {
        "Not enough participants"
    } else {
        v[n - 1].1
    }
}
