use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::*;

lazy_static! {
    static ref D: HashMap<u16, u16> = {
        let mut d = HashMap::new();
        for i in 1..60001 {
            d.insert(i, f(i));
        }
        d
    };
}

fn f(mut n: u16) -> u16 {
    let mut s = HashSet::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            s.insert(i);
            n /= i;
        }
        i += 1;
    }
    if n > 0 {
        s.insert(n);
    }
    s.iter().product()
}

fn hash_rad_seq(n: u16, k: usize) -> u16 {
    *D.iter()
        .filter(|t| t.0 <= &n)
        .sorted_by_key(|t| (t.1, t.0))
        .nth(k - 1)
        .unwrap()
        .0
}
