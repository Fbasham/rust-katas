fn divisions(n: u32, d: u32) -> u32 {
    if n < d || d == 0 {
        0
    } else {
        1 + divisions(n / d, d)
    }
}
