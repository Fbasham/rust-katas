fn fortune(f0: i32, p0: f64, c0: i32, n: i32, i0: f64) -> bool {
    let mut f = f0 as f64;
    let mut c = c0 as f64;
    let mut p = (p0 as f64) / 100.0;
    let mut i = (i0 as f64) / 100.0;
    for _ in 1..n {
        f = (f + p * f - c).floor();
        c = (c + c * i).floor();
    }
    f >= 0.0
}
