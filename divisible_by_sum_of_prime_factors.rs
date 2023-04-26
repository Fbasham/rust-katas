fn f(mut n: u32) -> u32 {
    let mut s = 0;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            s += i;
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        s += n;
    }
    s
}

fn mult_primefactor_sum(a: u32, b: u32) -> Vec<u32> {
    (a..=b).filter(|&i| i % f(i) == 0 && i != f(i)).collect()
}
