fn find_next_square(n: u64) -> Option<u64> {
    let m = (n as f64).sqrt();
    match m % 1.0 {
        0.0 => Some((m + 1.0).powf(2.0) as u64),
        _ => None,
    }
}
