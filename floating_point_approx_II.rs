fn interp(f: fn(f64) -> f64, l: f64, u: f64, n: i32) -> Vec<f64> {
    let d = (u - l) / (n as f64);
    (0..n)
        .map(|i| (f(l + (i as f64) * d) * 100.0).floor() / 100.0)
        .collect()
}
