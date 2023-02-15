fn persistence(n: u64) -> u64 {
    if n < 10 {
        0
    } else {
        1 + persistence(n.to_string().chars().map(|e| e as u64 - 48).product())
    }
}
