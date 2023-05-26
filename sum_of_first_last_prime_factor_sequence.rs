fn f(mut n: u32) -> u32 {
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
    if v.len() < 2 {
        0
    } else {
        v[0] + v[v.len() - 1]
    }
}

fn sflpf_data(k: u32, m: u32) -> Vec<u32> {
    let mut v = vec![];
    for i in 4..m {
        if f(i) == k {
            v.push(i)
        }
    }
    v
}
