fn sum_of_n(n: i32) -> Vec<i32> {
    (0..n.abs() + 1)
        .map(|i| (if n < 0 { -1 } else { 1 }) * i * (i + 1) / 2)
        .collect()
}
