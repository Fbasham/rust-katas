fn cooking_time(n: u32) -> u32 {
    (((n as f64) / 8.0).ceil() * 5.0) as u32
}
