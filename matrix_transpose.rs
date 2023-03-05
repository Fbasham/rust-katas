fn transpose(a: &[Vec<u8>]) -> Vec<Vec<u8>> {
    a[0].iter()
        .enumerate()
        .map(|(i, e)| a.iter().map(|v| v[i]).collect())
        .collect()
}
