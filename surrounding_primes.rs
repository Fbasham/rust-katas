fn f(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }
    for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn prime_bef_aft(n: u32) -> (u32, u32) {
    (
        (2..=n - 1).rev().find(|&i| f(i)).unwrap(),
        (n + 1..).find(|&i| f(i)).unwrap(),
    )
}
