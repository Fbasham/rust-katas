fn product_array(a: &[u64]) -> Vec<u64> {
    let p = a.iter().product::<u64>();
    a.iter().map(|e| p / e).collect()
}
