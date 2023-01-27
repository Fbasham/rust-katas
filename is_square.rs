fn is_square(n: i64) -> bool {
    (f64::sqrt(n as f64) as i64).pow(2) == n
}
