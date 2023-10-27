use std::collections::HashMap;

fn f(mut n: i32) -> HashMap<i32, i32> {
    let mut d = HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            if !d.contains_key(&i) {
                d.insert(i, 0);
            }
            d.insert(i, d.get(&i).unwrap() + 1);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        d.insert(n, 1);
    }
    d
}

fn zeroes(b: i32, n: i32) -> i32 {
    let mut m = i32::MAX;
    for (k, v) in f(b) {
        let mut x = 0;
        let mut i = 1;
        while n >= k.pow(i) {
            x += n / k.pow(i);
            i += 1;
        }
        m = m.min(x / v);
    }
    m
}
