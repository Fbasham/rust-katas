fn shortest_arrang(n: u32) -> Option<Vec<u32>> {
    for k in 2..n {
        for i in 1..n {
            let t = (k as f64) / 2.0 * (2.0 * (i as f64) + (k as f64) - 1.0);
            if t == n as f64 {
                return Some((i..i + k).rev().collect());
            }
        }
    }
    None
}
