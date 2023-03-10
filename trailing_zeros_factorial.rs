fn zeros(n: u64) -> u64 {
    if n <= 0 {
        0
    } else {
        n / 5 + zeros(n / 5)
    }
}
