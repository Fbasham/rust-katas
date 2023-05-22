fn solve(n: u64) -> Option<u64> {
    for i in 1..5000000 {
        let x = i * i;
        if ((n + x) as f64).sqrt() % 1.0 == 0.0 {
            return Some(x);
        }
    }
    None
}
