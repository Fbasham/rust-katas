use std::collections::HashSet;

fn f(n: u64) -> u64 {
    let mut s = HashSet::new();
    for i in 1..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            s.insert(i);
            s.insert(n / i);
        }
    }
    s.iter().map(|e| e * e).sum()
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..n)
        .map(|e| (e, f(e)))
        .filter(|&e| (e.1 as f64).sqrt() % 1.0 == 0.0)
        .collect()
}
