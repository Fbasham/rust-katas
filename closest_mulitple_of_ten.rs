fn closest_multiple_of_10(n: u32) -> u32 {
    (((n as f64) / 10.0).round() * 10.0) as u32
}
