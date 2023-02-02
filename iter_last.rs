fn last<T: Clone>(s: &[T]) -> T {
    s.iter().last().unwrap().clone()
}
