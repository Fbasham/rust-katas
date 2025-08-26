fn even_odd(a: &[i64]) -> i64 {
    a.iter()
        .enumerate()
        .fold(0, |a, (i, e)| if i % 2 == 0 { a + e } else { a * e })
}
