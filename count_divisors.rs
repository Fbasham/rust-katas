fn divisors(n: u32) -> u32 {
    let mut c = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            c += 2;
        }
        i += 1;
    }
    c - (if (n as f64).sqrt() % 1.0 == 0.0 { 1 } else { 0 })
}
