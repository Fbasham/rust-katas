fn chain<F: Fn(i32) -> i32>(i: i32, a: &[F]) -> i32 {
    a.iter().fold(i, |e, f| f(e))
}
