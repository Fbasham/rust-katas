fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }
    for i in 3..=(n as f64).sqrt() as i64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}
