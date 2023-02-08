fn pyramid(n: u16) -> u16 {
    (((1 + 8 * n as u32) as f64).sqrt() as u16 - 1) / 2
}
