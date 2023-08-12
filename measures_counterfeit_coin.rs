fn how_many_measurements(n: u64) -> u32 {
    ((n as f64).log(3.0)).ceil() as u32
}