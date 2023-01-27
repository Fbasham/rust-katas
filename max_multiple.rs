fn max_multiple(d: u32, b: u32) -> u32 {
    for i in (1..=b).rev() {
        if i % d == 0 {
            return i;
        }
    }
    0
}
