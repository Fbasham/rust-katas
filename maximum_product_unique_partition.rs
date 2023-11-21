fn maximum_product(n: u32) -> u64 {
    fn f(n: u64, i: u64) -> u64 {
        if i * (i + 1) / 2 < n {
            return 0;
        }
        return if n == 0 {
            1
        } else {
            f(n, i - 1).max(i * f(n - i, (n - i).min(i - 1)))
        };
    }
    return f(n as u64, n as u64);
}
