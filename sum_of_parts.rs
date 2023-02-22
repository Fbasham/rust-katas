fn parts_sums(a: &[u64]) -> Vec<u64> {
    let mut n = a.iter().sum();
    let mut v = vec![n];
    for i in 0..a.len() {
        n -= a[i];
        v.push(n);
    }
    v
}
