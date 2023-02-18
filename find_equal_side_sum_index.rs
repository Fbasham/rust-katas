fn find_even_index(a: &[i32]) -> Option<usize> {
    (0..a.len()).position(|i| a[..i].iter().sum::<i32>() == a[i + 1..].iter().sum::<i32>())
}
