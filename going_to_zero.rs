fn going(n: i32) -> f64 {
    let mut p = 1.0;
    let mut x = 1.0;
    for i in (0..=n).rev() {
        x *= i as f64;
        let t = p + 1.0 / x;
        if (t - p).abs() < 1e-8 {
            break;
        }
        p = t;
    }
    (p * 1000000.0 - 0.5).round() / 1000000.0
}
