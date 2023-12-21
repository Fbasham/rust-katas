use itertools::Itertools;

fn f(mut n: f64) -> Vec<i64> {
    let mut i = 2.0;
    let mut d = vec![];
    while i * i <= n {
        while n % i == 0.0 {
            d.push(i as i64);
            n /= i;
        }
        i += 1.0;
    }
    if n > 1.0 {
        d.push(n as i64)
    }
    d
}

fn c(k: i64) -> i32 {
    let n = k as f64;
    if n.sqrt() % 1.0 != 0.0 {
        return 0;
    }
    f(n.powf(1.5))
        .iter()
        .counts()
        .values()
        .fold(1, |a, c| a * (*c as i32 + 1))
}
