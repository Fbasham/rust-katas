fn score(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => (2.0_f64.powf((n as f64).log2().ceil()) - 1.0) as u32,
    }
}
