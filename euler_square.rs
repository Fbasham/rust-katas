fn create_euler_square(n: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let a: Vec<Vec<i32>> = (0..n)
        .map(|i| (0..n).map(|j| ((i + j) % n + 1) as i32).collect())
        .collect();
    let b = a.iter().cloned().rev().collect();
    (a, b)
}
