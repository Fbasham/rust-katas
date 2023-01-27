fn powers_of_two(n: u8) -> Vec<u128> {
    (0..=n).map(|e| u128::pow(2, e as u32)).collect()
}
