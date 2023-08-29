fn range_bit_count(a: u32, b: u32) -> u32 {
    (a..=b).map(|i| i.count_ones()).sum()
}
