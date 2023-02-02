fn modified_sum(a: &[i32], p: u32) -> i32 {
    a.iter().map(|e| e.pow(p)).sum::<i32>() - a.iter().sum::<i32>()
}
