fn max_sequence(a: &[i32]) -> i32 {
    let mut s = 0;
    let mut t = 0;
    for e in a {
        t = (t + e).max(0);
        s = s.max(t);
    }
    s
}
