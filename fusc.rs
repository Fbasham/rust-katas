fn fusc(n: u32) -> u32 {
    if n < 2 {
        n
    } else if n % 2 == 0 {
        fusc(n / 2)
    } else {
        fusc(n / 2) + fusc(n / 2 + 1)
    }
}
