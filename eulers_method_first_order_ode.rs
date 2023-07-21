use std::f64::consts::E;

fn ex_euler(n: i32) -> f64 {
    let h = 1.0 / (n as f64);
    let mut x = 0.0;
    let mut y = 1.0;
    let mut r = vec![];
    for _ in 0..=n {
        let z = 1.0 + 0.5 * E.powf(-4.0 * x) - 0.5 * E.powf(-2.0 * x);
        r.push((y - z).abs() / z);
        y = y + (2.0 - E.powf(-4.0 * x) - 2.0 * y) * h;
        x += h;
    }
    r.iter().sum::<f64>() / (n as f64 + 1.0)
}
