use std::collections::HashSet;
use itertools::Itertools;

fn is_prime(n: u32) -> bool {
    if n<2 {return false}
    if n%2==0 {return n==2}
    let mut i = 3;
    while i*i<=n {
        if n%i==0 {return false}
        i += 2
    }
    true
}

fn permutational_primes(n: u32, k: u32) -> (u32, u32, u32) {
    let mut v = HashSet::new();
    let mut u = HashSet::new();
    let mut c = 0;
    for i in 2..=n {
        if is_prime(i) && !v.contains(&i) {
            let s = i.to_string();
            let mut t = HashSet::new();
            for p in s.chars().permutations(s.len()) {
                let x = p.iter().join("").parse::<u32>().unwrap();
                if p[0]!='0' && x<n && is_prime(x) {
                    t.insert(x);
                }
            }
            if t.len() as u32==k+1 {
                c += 1;
                u.insert(i);
                for e in t.iter().cloned() {v.insert(e);}
            }
        }
    }
    if c==0 {return (0,0,0)}
    (c,*u.iter().min().unwrap(),*u.iter().max().unwrap())
}