fn array_leaders(a: &[i32]) -> Vec<i32> {
    a.iter()
        .enumerate()
        .filter(|&(i, e)| e > &a[i + 1..].iter().sum::<i32>())
        .map(|(i, e)| *e)
        .collect()
}
