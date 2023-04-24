fn f(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }
    for i in 3..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn check_goldbach(n: u32) -> Option<(u32, u32)> {
    if n % 2 == 1 || n < 2 {
        return None;
    }
    for i in 2..n {
        if f(i) && f(n - i) {
            return Some((i, n - i));
        }
    }
    None
}
