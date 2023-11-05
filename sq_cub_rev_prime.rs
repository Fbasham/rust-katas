use lazy_static::lazy_static;

lazy_static! {
    static ref A: Vec<u64> = {
        let mut v = vec![];
        let mut n: u64 = 89;
        while v.len()<250 {
            let s = n.pow(2).to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();
            let t = n.pow(3).to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();
            if is_prime(s) && is_prime(t) {v.push(n);}
            n += 1;
        }
        v
    };
}

fn is_prime(n: u64) -> bool {
    if n<2 {return false}
    if n%2==0 {return n==2}
    let mut i = 3;
    while i*i<=n {
        if n%i==0 {return false}
        i += 2;
    }
    true
}

fn sq_cub_rev_prime(n: u32) -> u32 {
    A[n as usize-1] as u32
}