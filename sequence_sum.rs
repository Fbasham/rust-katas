fn sequence_sum(m: u32, n: u32, k: u32) -> u32 {
    if n < m {
        return 0;
    }
    let mut r = 0;
    let mut i = m;
    while i <= n {
        r += i;
        i += k;
    }
    r
}
