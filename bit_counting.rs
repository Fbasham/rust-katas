fn count_bits(n: u32) -> u32 {
    return if n == 0 {
        0
    } else {
        (n & 1) + count_bits(n >> 1)
    };
}
