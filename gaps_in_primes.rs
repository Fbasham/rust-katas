fn f(n: u64) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    for i in 3..=(n as f64).sqrt().ceil() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    for i in m..n {
        if f(i) && f(i + g as u64) && (i..=i + g as u64).filter(|&e| f(e)).count() == 2 {
            return Some((i, i + g as u64));
        }
    }
    None
}
