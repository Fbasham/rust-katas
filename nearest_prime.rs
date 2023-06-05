fn f(n: u64) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn solve(n: u64) -> u64 {
    let mut i = 0;
    loop {
        if f(n - i) {
            return n - i;
        }
        if f(n + i) {
            return n + i;
        }
        i += 1
    }
}
