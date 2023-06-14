fn amort(rate: f64, balance: i64, term: i64, k: i32) -> String {
    let r = rate / 100.0 / 12.0;
    let mut b = balance as f64;
    let n = r * b;
    let d = 1.0 - (1.0 + r).powf(-term as f64);
    let c = n / d;
    let mut p = 0.0;
    let mut x = 0.0;
    for _ in 0..k {
        x = b * r;
        p = c - x;
        b -= p;
    }
    format!(
        "num_payment {k} c {} princ {} int {} balance {}",
        c.round(),
        p.round(),
        x.round(),
        b.round()
    )
}
