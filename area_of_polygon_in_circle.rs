fn area_of_polygon_inside_circle(r: f64, n: u32) -> f64 {
    (n as f64) / 2.0 * r.powf(2.0) * (std::f64::consts::PI * 2.0 / (n as f64)).sin()
}
