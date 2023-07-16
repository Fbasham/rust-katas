fn solution(n: u8, b: u32) -> Vec<u32> {
    (0..2_u32.pow(n as u32))
        .filter(|&e| b > 0 && e & b != 0)
        .collect()
}
