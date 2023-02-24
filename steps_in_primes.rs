fn f(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 || n < 2 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn step(k: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    for i in m..=n {
        if f(i) && f(i + k as u64) {
            return Some((i, i + k as u64));
        }
    }
    None
}
