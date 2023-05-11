use std::f64::consts::PI;

fn f(x: f64) -> f64 {
    x.sin().powf(3.0) * 3.0 / 2.0
}

fn simpson(n: i32) -> f64 {
    let h = PI / (n as f64);
    let mut x = 0.0;
    for i in 1..=n / 2 {
        x += f((2.0 * (i as f64) - 1.0) * h)
    }
    let mut y = 0.0;
    for i in 1..=n / 2 - 1 {
        y += f(2.0 * (i as f64) * h)
    }
    PI / 3.0 / (n as f64) * (f(0.0) + f(PI) + 4.0 * x + 2.0 * y)
}
