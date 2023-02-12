fn alternate<'a>(n: usize, x: &'a str, y: &'a str) -> Vec<&'a str> {
    (0..n).map(|i| if i % 2 == 0 { x } else { y }).collect()
}
