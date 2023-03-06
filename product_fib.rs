fn product_fib(p: u64) -> (u64, u64, bool) {
    let mut a = 0;
    let mut b = 1;
    while a * b < p {
        (a, b) = (b, a + b)
    }
    (a, b, a * b == p)
}
