fn beggars(a: &[u32], n: usize) -> Vec<u32> {
    let mut v = vec![];
    for i in 0..n {
        let mut c = 0;
        for j in (i..a.len()).step_by(n) {
            c += a[j];
        }
        v.push(c);
    }
    v
}