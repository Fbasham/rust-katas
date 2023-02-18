fn multiplication_table(n: usize) -> Vec<Vec<usize>> {
    (1..=n).map(|i| (1..=n).map(|j| i * j).collect()).collect()
}
