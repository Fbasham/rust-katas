fn doubles(K: i32, N: i32) -> f64{
    (1..=K).map(|k| (1..=N).map(|n| 1.0/(k as f64*(n as f64+1.0).powf(2.0*k as f64))).sum::<f64>()).sum()
}