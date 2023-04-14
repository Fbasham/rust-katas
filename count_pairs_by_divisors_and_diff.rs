use std::collections::HashSet;

fn f(n: u32) -> u32 {
    let mut s = HashSet::new();
    for i in 1..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            s.insert(i);
            s.insert(n / i);
        }
    }
    s.len() as u32
}

fn count_pairs_int(k: u32, n: u32) -> u32 {
    let mut c = 0;
    for i in 1..n - k {
        if f(i) == f(i + k) {
            c += 1;
        }
    }
    c
}
