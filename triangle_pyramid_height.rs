fn find_height(n: usize) -> u16 {
    (0..).find(|i| i * (i + 1) * (i + 2) / 6 > n).unwrap() as u16 - 1
}
