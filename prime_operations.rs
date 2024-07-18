use std::collections::HashMap;
use itertools::Itertools;

fn f(mut n: u64) -> HashMap<u64,u32> {
    let mut d = HashMap::new();
    let mut i = 2;
    while i*i<=n {
        while n%i==0 {
            d.entry(i).and_modify(|e| *e += 1).or_insert(1);
            n /= i;
        }
        i += 1;
    } 
    if n>1 {d.insert(n,1);}
    d
}

fn prime_operations(x: u64, y: u64) -> u32 {
    let dx = f(x);
    let dy = f(y);
    dx.keys().chain(dy.keys()).unique().fold(0,|a,c| a+(*dx.get(c).unwrap_or(&0) as f64-*dy.get(c).unwrap_or(&0) as f64).abs() as u32)
}