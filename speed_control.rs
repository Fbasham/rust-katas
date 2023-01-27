use std::iter::zip;

fn gps(s: i32, v: Vec<f64>) -> i32 {
    zip(v.iter(), v.iter().skip(1))
        .map(|(i, j)| ((j - i) * 3600.0 / (s as f64)) as i32)
        .max()
        .unwrap_or(0)
}
