fn f(mut n: u32) -> u32 {
    let mut r = 0;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            r += i;
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        r += n
    }
    r
}

fn g(n: u32) -> u32 {
    let mut r = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            r += i + (if n / i == i { 0 } else { n / i })
        }
        i += 1;
    }
    r
}

fn ds_multof_pfs(a: u32, b: u32) -> Vec<u32> {
    (a..=b).filter(|&e| g(e) % f(e) == 0).collect()
}
