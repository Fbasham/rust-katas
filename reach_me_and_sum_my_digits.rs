fn sum_dig_nth_term(n: u32, a: &[u32], k: usize) -> u32 {
    (0..k - 1)
        .fold(n, |x, i| x + a[i % (a.len())])
        .to_string()
        .chars()
        .map(|e| e.to_digit(10).unwrap())
        .sum()
}
