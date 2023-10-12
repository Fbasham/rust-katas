fn shortest_distance(a: f64, b: f64, c: f64) -> f64 {
    (a * a + b * b + c * c + 2.0 * a * b)
        .sqrt()
        .min((a * a + b * b + c * c + 2.0 * a * c).sqrt())
        .min((a * a + b * b + c * c + 2.0 * b * c).sqrt())
}
