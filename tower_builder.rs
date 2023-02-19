fn tower_builder(n: usize) -> Vec<String> {
    (0..n)
        .map(|i| " ".repeat(n - i - 1) + &"*".repeat(2 * i + 1) + &" ".repeat(n - i - 1))
        .collect()
}
