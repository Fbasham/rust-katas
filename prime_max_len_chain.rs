use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::*;

lazy_static! {
    static ref A: Vec<u32> = { (2..500_000).filter(|i| is_prime(*i)).collect() };
}

fn is_prime(n: u32) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    for i in 3..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn prime_maxlength_chain(n: u32) -> Vec<u32> {
    let a = A.iter().filter(|&e| e < &n).collect::<Vec<_>>();
    let mut t = (0..a.len()).map(|_| 0).collect::<Vec<_>>();
    for (i, e) in a.iter().enumerate() {
        if i == 0 {
            t[i] = **e
        } else {
            t[i] = t[i - 1].saturating_add(**e)
        }
    }
    t.insert(0, 0);
    let mut d: HashMap<usize, HashSet<u32>> = HashMap::new();
    let mut m = 0;
    for i in 0..t.len() - 1 {
        for j in i + 1..t.len() {
            let k = j - i;
            let v = t[j] - t[i];
            if v > n {
                break;
            }
            if v < n && is_prime(v) {
                d.entry(k).or_insert(HashSet::new()).insert(v);
                m = m.max(k);
            }
        }
    }
    d.get(&m).unwrap().iter().cloned().sorted().collect()
}
