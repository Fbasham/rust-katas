fn all_non_consecutive(a: &[i32]) -> Vec<(usize, i32)> {
    (1..a.len())
        .map(|i| {
            if a[i] - a[i - 1] != 1 {
                (i, a[i])
            } else {
                (0, 0)
            }
        })
        .filter(|e| e.0 != 0)
        .collect::<Vec<(usize, i32)>>()
}
