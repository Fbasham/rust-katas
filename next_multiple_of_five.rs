fn round_to_next_5(n: i32) -> i32 {
    ((n as f64 / 5.0).ceil() * 5.0) as i32
}
