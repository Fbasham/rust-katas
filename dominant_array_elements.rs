fn solve(a: &[u32]) -> Vec<u32> {
    a.iter()
        .enumerate()
        .filter(|&(i, e)| a[i + 1..].iter().all(|x| e > x))
        .map(|e| *e.1)
        .collect()
}
