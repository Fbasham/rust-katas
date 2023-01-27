fn digital_root(n: i64) -> i64 {
    let mut t = n;
    while t > 9 {
        t = t.to_string().chars().map(|e| (e as i64) - 48).sum();
    }
    t
}
