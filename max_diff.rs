fn max_diff(a: &[i32]) -> i32 {
    a.iter().max().unwrap_or(&0) - a.iter().min().unwrap_or(&0)
}
