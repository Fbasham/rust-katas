fn perimeter(n: u64) -> u64 {
    let mut a = 1;
    let mut b = 1;
    let mut p = 0;
    for _ in 0..=n {
        p += a;
        (a, b) = (b, a + b);
    }
    p * 4
}
