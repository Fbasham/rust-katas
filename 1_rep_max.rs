fn calculate_1_rm(W: i32, R: i32) -> i32 {
    let w = W as f64;
    let r = R as f64;
    match R {
        0 => 0,
        1 => W,
        _ => ((w * (1.0 + r / 30.0))
            .max(100.0 * w / (101.3 - 2.67123 * r))
            .max(w * r.powf(0.1)))
        .round() as i32,
    }
}
