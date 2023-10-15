fn find_sum(a: &[Vec<u32>]) -> u32 {
    let mut v = (0..=a.len())
        .map(|_| (0..=a[0].len()).map(|_| 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 1..=a.len() {
        for j in 1..=a[0].len() {
            v[i][j] = v[i - 1][j].max(v[i][j - 1]) + a[i - 1][j - 1];
        }
    }
    v[a.len()][a[0].len()]
}
