fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 || n < 2 {
        return false;
    }
    for i in (3..=(n as f64).sqrt().floor() as u64).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn next_prime(mut n: u64) -> u64 {
    n += 1;
    loop {
        if is_prime(n) {
            return n;
        }
        n += 1
    }
}
