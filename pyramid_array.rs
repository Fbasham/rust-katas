fn pyramid(n: usize) -> Vec<Vec<u32>> {
    (0..n).map(|i| (0..i + 1).map(|e| 1).collect()).collect()
}
