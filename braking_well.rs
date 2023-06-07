fn dist(v: f64, mu: f64) -> f64 {
    v / 3.6 + v / 3.6 * v / 3.6 / 2.0 / mu / 9.81
}

fn speed(d: f64, mu: f64) -> f64 {
    let a = 1.0 / 254.2752 / mu;
    let b = 1.0 / 3.6;
    (-b + (b * b + 4.0 * a * d).sqrt()) / 2.0 / a
}
