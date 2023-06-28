fn f(mut n: i32) -> i32 {
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
        c += 1
    }
    c
}

fn consec_kprimes(k: i32, v: Vec<i32>) -> i32 {
    v.iter()
        .zip(&v[1..])
        .filter(|&(&i, &j)| f(i) == k && f(j) == k)
        .count() as i32
}
