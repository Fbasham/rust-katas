fn prime_factors(mut n: u32) -> Vec<u32> {
    let mut v = vec![];
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            v.push(i);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        v.push(n)
    }
    v
}
