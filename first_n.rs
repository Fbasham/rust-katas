fn multiples(m: i32, n: f64) -> Vec<f64> {
    (1..=m).map(|i| (i as f64) * n).collect()
}
