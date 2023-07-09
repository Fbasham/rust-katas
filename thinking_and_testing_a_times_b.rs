fn test_it(a: u64, b: u64) -> u64 {
    a.to_string().chars().map(|e| e as u64 - 48).sum::<u64>()
        * b.to_string().chars().map(|e| e as u64 - 48).sum::<u64>()
}
