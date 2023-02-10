fn hamming(a: &str, b: &str) -> usize {
    let x = a.chars().collect::<Vec<_>>();
    let y = b.chars().collect::<Vec<_>>();
    (0..a.len()).map(|i| if x[i] != y[i] { 1 } else { 0 }).sum()
}
