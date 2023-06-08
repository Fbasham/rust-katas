use std::collections::HashSet;

fn f(n: u32) -> u8 {
    let mut s = HashSet::new();
    for i in 1..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            s.insert(i);
            s.insert(n / i);
        }
    }
    s.len() as u8
}

fn find_min_num(n: u8) -> u32 {
    (0..).find(|e| f(*e as u32) == n).unwrap()
}
