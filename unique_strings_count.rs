use num::bigint::BigUint;
use itertools::Itertools;

fn f(n: usize) -> BigUint {
    if n<1 {BigUint::from(1 as usize)} else {BigUint::from(n)*f(n-1)}
}

fn uniq_count(s: &str) -> BigUint {
    let d = s.to_lowercase().chars().counts().values().filter(|&e| e>&(1 as usize)).fold(BigUint::from(1 as usize),|a,c| a*f(*c));
    f(s.len())/d
}