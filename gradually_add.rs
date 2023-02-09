fn add(a: &[i64]) -> i64 {
    a.iter().enumerate().map(|(i, e)| e * (i as i64 + 1)).sum()
}
