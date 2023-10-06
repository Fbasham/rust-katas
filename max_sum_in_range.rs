fn max_sum(a: &[i32], r: &[(usize, usize)]) -> i32 {
    let mut t = vec![0; a.len() + 1];
    for i in 0..a.len() {
        t[i + 1] = t[i] + a[i];
    }
    r.iter().map(|(i, j)| t[j + 1] - t[*i]).max().unwrap()
}
