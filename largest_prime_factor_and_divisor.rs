fn pf(mut n: i32) -> u32 {
    let mut m = 0;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            m = m.max(i);
            n /= i;
        }
        i += 1;
    }
    m.max(n) as u32
}

fn d(n: i32) -> i32 {
    (2..=(n as f64).sqrt() as i32)
        .find(|i| n.abs() % i == 0)
        .unwrap_or(1)
}

fn big_prime_fac_div(n: i32) -> Option<(u32, u32)> {
    let x = d(n.abs());
    if x == 1 {
        None
    } else {
        Some((pf(n.abs()), (n.abs() / x) as u32))
    }
}
