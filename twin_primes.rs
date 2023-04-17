fn f(n: i32) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn twin_prime(n: i32) -> u32 {
    (2..n)
        .map(|i| if f(i) && f(i + 2) { 1 as u32 } else { 0 as u32 })
        .sum::<u32>()
}
