fn largest_difference(a: &[i32]) -> usize {
    let mut m = 0;
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if a[j] >= a[i] {
                m = m.max(j - i);
            }
        }
    }
    m
}
