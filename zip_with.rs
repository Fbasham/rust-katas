fn zip_with<F, T, S, R>(f: F, a: &[T], b: &[S]) -> Vec<R>
where
    F: Fn(T, S) -> R,
    T: Copy,
    S: Copy,
{
    a.iter()
        .zip(b)
        .map(|(i, j)| f(i.clone(), j.clone()))
        .collect()
}
