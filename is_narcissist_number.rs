fn narcissistic(n: u64) -> bool {
    let s = n.to_string();
    s.chars()
        .map(|e| (e as u64 - 48).pow(s.len() as u32))
        .sum::<u64>()
        == n
}
