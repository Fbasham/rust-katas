fn finance(n: u64) -> u64 {
    (0..=n).fold(0, |a, c| a + (c + 1) * (c + 2 * c) / 2)
}
