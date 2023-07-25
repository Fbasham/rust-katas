use std::collections::HashMap;

fn pf(mut n: i32) -> HashMap<i32, i32> {
    let mut d = HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            d.insert(i, d.get(&i).unwrap_or(&0) + 1);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        d.insert(n, 1);
    }
    d
}

fn f(n: i32) -> i32 {
    pf(n)
        .iter()
        .map(|(k, v)| v * k.pow(*v as u32 - 1))
        .product()
}
