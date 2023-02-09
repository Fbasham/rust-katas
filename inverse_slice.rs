fn inverse_slice<T: Clone>(a: &[T], i: usize, j: usize) -> Vec<T> {
    (0..a.len())
        .filter(|&e| e < i || e >= j)
        .map(|e| a[e].clone())
        .collect::<Vec<T>>()
}
