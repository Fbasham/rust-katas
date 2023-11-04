fn sort_by_bit(a: &[u8]) -> u32 {
    a.iter().map(|e| 2_u32.pow(*e as u32)).sum()
}
