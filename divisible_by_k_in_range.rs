fn divisible_count(x: u64, y: u64, k: u64) -> u64 {
    y / k - x / k + (if x % k == 0 { 1 } else { 0 })
}
