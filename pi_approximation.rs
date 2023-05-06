use std::f64::consts::PI;

fn rnd10(f: f64) -> f64 {
    (f * 1e10).round() / 1e10
}

fn iter_pi(e: f64) -> (i32, f64) {
    let mut c = 0.0;
    let mut n = 0.0;
    let mut d = 1.0;
    while (4.0 * n - PI).abs() > e {
        n += (-1.0_f64).powf(c) / d;
        c += 1.0;
        d += 2.0;
    }
    (c as i32, rnd10(4.0 * n))
}
