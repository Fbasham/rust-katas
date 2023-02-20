fn two_sum(a: &[i32], n: i32) -> (usize, usize) {
    for i in 0..a.len() - 1 {
        for j in i + 1..a.len() {
            if a[i] + a[j] == n {
                return (i, j);
            }
        }
    }
    (0, 0)
}
