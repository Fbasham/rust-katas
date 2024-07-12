fn is_prime(n: u64) -> bool {
    if n%2==0 {return n==2}
    let mut i = 3;
    while i*i<=n {
        if n%i==0 {return false}
        i += 2;
    }
    true
}

fn solution(m: u64, n: u64) -> Vec<u64> {
    let mut v = vec![];
    let mut p: u64 = 2;
    while p.pow(4)<=n {
        let x = p.pow(4);
        if is_prime(p) && x>=m && x<=n {v.push(x)}
        p += 1;
    }
    v
}