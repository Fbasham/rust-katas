fn bubblesort_once(x: &[u32]) -> Vec<u32> {
    let mut a = x.iter().map(|e| *e).collect::<Vec<_>>();
    for i in 0..a.len() - 1 {
        if a[i] > a[i + 1] {
            (a[i], a[i + 1]) = (a[i + 1], a[i])
        }
    }
    a.to_vec()
}
