fn solve(sum: u32, gcd: u32) -> Option<(u32, u32)> {
    if sum % gcd > 0 {
        None
    } else {
        Some((gcd, sum - gcd))
    }
}
