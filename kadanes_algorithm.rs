fn largest_sum(a: &[i32]) -> i32 {
    let mut r = 0;
    let mut m = 0;
    for e in a {
        m = (m + e).max(0);
        r = r.max(m);
    }
    r
}
