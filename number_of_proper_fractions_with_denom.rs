use itertools::Itertools;
use std::collections::*;

fn f(mut n: u64) -> HashMap<u64, usize> {
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
        v.push(n);
    }
    v.iter().cloned().counts()
}

fn proper_fractions(n: u64) -> u64 {
    let d = f(n);
    if n == 1 {
        0
    } else {
        d.iter()
            .map(|(k, v)| k.pow(*v as u32 - 1) * (*k - 1))
            .product()
    }
}
