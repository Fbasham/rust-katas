fn is_perfect_power(n: u64) -> Option<(u64, u32)> {
    for i in 1..1000 {
        let j = (((n as f64).ln() / (i as f64).ln()) + 0.0001).floor();
        if j > 1.0 && (i as u64).pow(j as u32) == n {
            return Some((i as u64, j as u32));
        }
    }
    None
}
