fn corner_circle(r: f64) -> f64 {
    ((r * ((2.0_f64).sqrt() - 1.0).powf(2.0)) * 100.0).round() / 100.0
}
