fn min_sum(a: &[u64]) -> u64 {
    let mut v = a.to_vec();
    v.sort();
    (0..v.len() / 2).map(|i| v[i] * v[v.len() - i - 1]).sum()
}
