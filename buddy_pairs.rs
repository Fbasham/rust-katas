use std::collections::HashSet;

fn f(n: i64) -> i64 {
    let mut s = HashSet::new();
    for i in 1..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            s.insert(i);
            s.insert(n / i);
        }
    }
    s.iter().sum::<i64>() - n
}

fn buddy(x: i64, y: i64) -> Option<(i64, i64)> {
    for i in x..=y {
        let s = f(i);
        if f(s - 1) - 1 == i && i < s - 1 {
            return Some((i, s - 1));
        }
    }
    None
}
