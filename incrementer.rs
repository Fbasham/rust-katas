fn incrementer(a: &[u32]) -> Vec<u32> {
    (1..a.len() + 1)
        .map(|i| (a[i - 1] + (i as u32)) % 10)
        .collect()
}
