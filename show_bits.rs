fn show_bits(n: i32) -> [u8; 32] {
    format!("{:032b}", n)
        .chars()
        .map(|e| e as u8 - 48)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
