use lazy_static::lazy_static;
use num::bigint::BigInt;

lazy_static! {
    static ref A: Vec<(u32, u32)> = {
        let mut v = vec![];
        let mut a = BigInt::from(0);
        let mut b = BigInt::from(1);
        let mut c = BigInt::from(1);
        let mut d = BigInt::from(2);
        let mut e = BigInt::from(4);
        for i in 1..10000 {
            let s = a.to_string();
            if s.len() > 8 {
                let t = s[s.len() - 9..].parse::<u32>().unwrap();
                if f(t) {
                    v.push((i, t));
                }
            }
            (a, b, c, d, e) = (
                b.clone(),
                c.clone(),
                d.clone(),
                e.clone(),
                d.clone() + e.clone() - c.clone() + b.clone() - a.clone(),
            );
        }
        v
    };
}

fn f(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn kth_last_digit_prime(k: u32) -> (u32, u32) {
    A[k as usize - 1]
}
