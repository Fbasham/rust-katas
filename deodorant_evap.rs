fn evaporator(_c: f64, p: i32, t: i32) -> i32 {
    (((t as f64) / 100.0).log2() / (1.0 - (p as f64) / 100.0).log2()).ceil() as i32
}
