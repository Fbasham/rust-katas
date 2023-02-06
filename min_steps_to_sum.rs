fn minimum_steps(a: &[i32], k: i32) -> usize {
    let mut t = a.to_vec();
    let mut s = 0;
    t.sort();
    for i in 0..a.len() {
        s += t[i];
        if s >= k {
            return i;
        }
    }
    0
}
