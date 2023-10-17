use std::f64;

fn crusoe(n: i32, mut d: f64, x: i32, dd: f64, da: f64) -> (f64, f64) {
    let mut a = (x as f64) * f64::consts::PI / 180.0;
    let mut x = 0.0;
    let mut y = 0.0;
    for _ in 0..n {
        x += a.cos() * d;
        y += a.sin() * d;
        a *= da;
        d *= dd;
    }
    (x, y)
}
