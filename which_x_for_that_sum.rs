fn solve(n: f64) -> f64 {
    let a = n;
    let b = -2.0 * n - 1.0;
    let c = n;
    (-b - (b * b - 4.0 * a * c).sqrt()) / 2.0 / a
}
