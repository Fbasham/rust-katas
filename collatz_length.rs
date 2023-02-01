fn collatz(n: u64) -> u64 {
    if n == 1 {
        1
    } else {
        1 + collatz(if n % 2 == 0 { n / 2 } else { 3 * n + 1 })
    }
}
