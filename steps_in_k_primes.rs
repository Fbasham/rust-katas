fn f(mut n: u64) -> u64 {
    let mut c = 0;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            c += 1;
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        c += 1;
    }
    c
}

fn kprimes_step(k: i32, x: i32, m: u64, n: u64) -> Option<Vec<(u64, u64)>> {
    let mut v = vec![];
    for i in m..n - (x as u64) {
        if f(i) == (k as u64) && f(i + (x as u64)) == (k as u64) {
            v.push((i, i + (x as u64)))
        }
    }
    match v.len() > 0 {
        true => Some(v),
        false => None,
    }
}
