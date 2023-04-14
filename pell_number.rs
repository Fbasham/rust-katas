fn pell(n: u32) -> u128 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        (a, b) = (b, 2 * b + a);
    }
    a
}
