fn f(n: usize) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn num_primorial(n: usize) -> u64 {
    let mut p = 1;
    let mut m = 1;
    let mut c = 0;
    while c <= n {
        while !f(m) {
            m += 1;
        }
        c += 1;
        p *= m;
        m += 1;
    }
    p as u64
}
