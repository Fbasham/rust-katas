fn fit_in(a: u32, b: u32, m: u32, n: u32) -> bool {
    m >= a + b && n >= a.max(b) || n >= a + b && m >= a.max(b)
}
