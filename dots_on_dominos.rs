fn dots_on_domino_bones(n: i32) -> Option<u64> {
    Some(n as u64 * (n as u64 + 1) * (n as u64 + 2) / 2)
}
