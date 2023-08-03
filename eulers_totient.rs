fn f(mut n: u64) -> Vec<u64> {
    let mut v = vec![];
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            if !v.contains(&i) {
                v.push(i)
            }
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        v.push(n)
    }
    v
}

fn totient(n: u64) -> usize {
    ((n as f64)
        * f(n)
            .iter()
            .cloned()
            .map(|e| 1.0 - 1.0 / (e as f64))
            .product::<f64>()
        + 0.00001) as usize
}
