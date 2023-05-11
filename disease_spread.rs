fn epidemic(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64) -> i32 {
    let mut m: f64 = 0.0;
    let mut s = s0 as f64;
    let mut i = i0 as f64;
    let mut r = 0.0;
    let dt = (tm as f64) / (n as f64);
    for x in 0..n {
        (s, i, r) = (
            s - dt * b * s * i,
            i + dt * (b * s * i - a * i),
            r + dt * i * a,
        );
        m = m.max(i);
    }
    m as i32
}
