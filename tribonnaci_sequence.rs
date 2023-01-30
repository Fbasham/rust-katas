fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut v = signature.to_vec();
    while v.len() < n {
        v.push(v[v.len() - 3..].iter().sum());
    }
    v[..n].to_vec()
}
