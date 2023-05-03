fn split_and_add(arr: &[u32], n: usize) -> Vec<u32> {
    let mut v = arr.to_vec();
    for _ in 0..n {
        let k = v.len() / 2;
        let mut a = v[..k].to_vec();
        let b = v[k..].to_vec();
        if a.len() != b.len() {
            a.insert(0, 0);
        }
        v = a.iter().zip(b.clone()).map(|t| t.0 + t.1).collect();
    }
    v
}
